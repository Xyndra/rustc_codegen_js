//! Control-flow structure recovery.
//!
//! MIR is a CFG with arbitrary edges; JS `if`/`while`/`switch` are nested.
//! This pass reconstructs the nesting so the emitter can render `if`/`else if`,
//! `while` and `switch` directly instead of a `bb`-dispatched state machine:
//!
//! 1. a postorder + dominator computation (Cooper-Harvey-Kennedy) gives
//!    `dominates` and, via the lowest common dominator of a branch's
//!    successors, its join block;
//! 2. back edges (edges to a dominator) give natural loops, whose bodies
//!    back-reachability computes;
//! 3. a planner walks the body and turns it into a `Node` tree: `Block`s,
//!    `Branch`es, `Loop`s and `continue`/`break` `Jump`s.
//!
//! Bodies that do not satisfy the planning rules (`None`) fall back to the
//! state machine, so the structured path is only taken when it is known to be
//! faithful: a loop with more than one exit, a join outside the enclosing
//! loop, an unmappable terminator, or a plan that would duplicate code beyond
//! the budget.

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use rustc_middle::mir::{BasicBlock, Body, TerminatorKind};

/// "No block" marker.
pub const NONE: usize = usize::MAX;
/// Join marker meaning "every path from here leaves the function" (the
/// virtual exit node the post-dominator tree is rooted at).
pub const VIRTUAL_EXIT: usize = usize::MAX - 1;

/// `JS_TIMING=1` prints exact per-phase timings to stderr. Every phase logs
/// START first and DONE with elapsed ms after, so a hang is identified by
/// the last START with no matching DONE.
fn timing_on() -> bool {
    std::env::var("JS_TIMING").is_ok()
}

fn ms(t: &Instant) -> String {
    format!("{:.3}ms", t.elapsed().as_secs_f64() * 1000.0)
}

/// A structured emission plan for one body.
pub enum Node {
    /// Render the block's statements and the effect part of its terminator
    /// (call assignment, drop, assert, throw, return).
    Block(usize),
    /// `if`/`else if` (or a JS `switch` for many arms) built from the
    /// block's `SwitchInt`. The plan continues at the branch's join block,
    /// which is the next node in the enclosing list.
    Branch {
        bb: usize,
        arms: Vec<(u128, Vec<Node>)>,
        otherwise: Vec<Node>,
    },
    /// `L: while (true) { <header stmts>; if (!(test)) break L; body }`
    /// (`loop` has no test). The plan continues at the loop's exit block,
    /// again the next node in the enclosing list.
    Loop { bb: usize, body: Vec<Node> },
    /// `continue`/`break` out to the loop `depth` levels out.
    Jump { depth: usize, is_break: bool },
    /// The path ends here (return / throw / unreachable).
    Stop,
}

/// Dominators + natural loops for one body.
pub struct Structure {
    po_num: Vec<usize>,
    /// Reachable from the entry block (unreachable blocks never merge).
    reachable: Vec<bool>,
    #[allow(dead_code)]
    header: Vec<usize>,
    /// Blocks that are loop headers.
    pub is_header: Vec<bool>,
    /// Loop bodies, keyed by header.
    bodies: HashMap<usize, HashSet<usize>>,
    /// Immediate post-dominator per block (NONE when the block cannot reach
    /// an exit, `VIRTUAL_EXIT` when the merge is the function's exit), from
    /// the reversed graph rooted at a virtual exit node.
    ipdom: Vec<usize>,
}

// Successors a structured walk may follow. Cleanup blocks are never entered
// in this backend (unwinding happens as inline panics), so their edges are
// dropped. Edges into `unreachable` blocks are dropped too: control never
// arrives there, and keeping the edge poisons join computation (an
// `otherwise: bb_unreachable` arm is an exit edge, so every branch's join
// collapses to the virtual exit and shared join blocks read as "reachable on
// two paths"). The planner still walks such arms explicitly when they are an
// `otherwise` target and renders them as `throw __unreachable()`.
fn successors(body: &Body<'_>, b: usize) -> Vec<usize> {
    let mut out = vec![];
    if let Some(term) = &body.basic_blocks[BasicBlock::from_usize(b)].terminator {
        match &term.kind {
            TerminatorKind::Goto { target } => out.push(target.index()),
            TerminatorKind::SwitchInt { targets, .. } => {
                for (_, t) in targets.iter() {
                    out.push(t.index());
                }
                // `iter()` stops short of the otherwise case.
                out.push(targets.otherwise().index());
            }
            TerminatorKind::Call { target, .. } => {
                if let Some(t) = target {
                    out.push(t.index());
                }
            }
            TerminatorKind::Drop { target, .. }
            | TerminatorKind::Assert { target, .. }
            | TerminatorKind::FalseEdge {
                real_target: target,
                ..
            }
            | TerminatorKind::FalseUnwind {
                real_target: target,
                ..
            } => out.push(target.index()),
            _ => {}
        }
    }
    out.retain(|&s| {
        let data = &body.basic_blocks[BasicBlock::from_usize(s)];
        if data.is_cleanup {
            return false;
        }
        !matches!(
            data.terminator.as_ref().map(|t| &t.kind),
            Some(TerminatorKind::Unreachable)
        )
    });
    out
}

fn dominates(a: usize, b: usize, idom: &[usize]) -> bool {
    let mut x = b;
    // Capped: a cyclic idom chain must degrade to `false` (the loop is then
    // missed and planning falls back) rather than hang the backend.
    let cap = 2 * idom.len() + 8;
    for _ in 0..cap {
        if x == a {
            return true;
        }
        if x == NONE || x == idom[x] {
            return false;
        }
        x = idom[x];
    }
    false
}

fn intersect(a: usize, b: usize, idom: &[usize], po_num: &[usize]) -> usize {
    let (mut a, mut b) = (a, b);
    // Postorder numbers: the entry block finishes last, so it has the
    // LARGEST number, and idom[entry] == entry (self-loop). The deeper node
    // (SMALLER po_num) must walk up; walking the larger one up spins on the
    // entry self-loop forever. Capped so malformed input returns NONE
    // (caller falls back) instead of hanging.
    let cap = 2 * idom.len() + 8;
    let mut steps = 0;
    while a != b {
        steps += 1;
        if steps > cap || a == NONE || b == NONE {
            return NONE;
        }
        if po_num[a] < po_num[b] {
            a = idom[a];
        } else {
            b = idom[b];
        }
    }
    a
}

/// Immediate post-dominators for the graph described by `succs`, with a
/// virtual exit node (`exit`) that every block without successors flows
/// into. Post-dominance is dominance on the graph with its edges reversed,
/// rooted at that exit.
fn post_dominators(succs: &[Vec<usize>], exit: usize) -> Option<Vec<usize>> {
    let t0 = Instant::now();
    let timing = timing_on();
    if timing {
        eprintln!("[js-timing] postdoms START blocks={}", succs.len());
    }
    let n = succs.len();
    // Reverse graph: an edge `b -> s` becomes `s -> b`; a block without
    // successors flows into `exit`, so `exit -> b` appears here.
    let mut rsucc = vec![vec![]; n + 1];
    let mut rpred = vec![vec![]; n + 1];
    for (b, ss) in succs.iter().enumerate() {
        if ss.is_empty() {
            rsucc[exit].push(b);
            rpred[b].push(exit);
        }
        for &s in ss {
            rsucc[s].push(b);
            rpred[b].push(s);
        }
    }
    // Postorder from the exit over the reversed edges.
    let mut po = vec![];
    let mut seen = vec![false; n + 1];
    let mut stack = vec![exit];
    while let Some(&b) = stack.last() {
        if !seen[b] {
            seen[b] = true;
            for &s in &rsucc[b] {
                if !seen[s] {
                    stack.push(s);
                }
            }
        } else {
            stack.pop();
            po.push(b);
        }
    }
    let mut po_num = vec![NONE; n + 1];
    for (i, &b) in po.iter().enumerate() {
        po_num[b] = i;
    }
    let order: Vec<usize> = {
        let mut o = po.clone();
        o.reverse();
        o
    };
    let mut idom = vec![NONE; n + 1];
    idom[exit] = exit;
    let mut changed = true;
    let mut passes = 0usize;
    while changed {
        passes += 1;
        if passes > n + 2 {
            // No convergence: give up (caller falls back) instead of hanging.
            if timing {
                eprintln!(
                    "[js-timing] postdoms NO-CONVERGE passes={passes} elapsed={}",
                    ms(&t0)
                );
            }
            return None;
        }
        changed = false;
        for &b in &order {
            let mut new = NONE;
            for &p in &rpred[b] {
                if idom[p] == NONE {
                    continue;
                }
                new = if new == NONE {
                    p
                } else {
                    let m = intersect(new, p, &idom, &po_num);
                    if m == NONE {
                        break;
                    }
                    m
                };
            }
            if new != NONE && idom[b] != new {
                idom[b] = new;
                changed = true;
            }
        }
    }
    if timing {
        eprintln!(
            "[js-timing] postdoms DONE passes={passes} elapsed={}",
            ms(&t0)
        );
    }
    Some(idom.into_iter().take(n).collect())
}

/// Map the virtual exit node (index `n`) onto `VIRTUAL_EXIT`.
fn fold_exit(v: usize, n: usize) -> usize {
    if v == n {
        VIRTUAL_EXIT
    } else {
        v
    }
}

impl Structure {
    pub fn analyze(body: &Body<'_>) -> Option<Structure> {
        if std::env::var("JS_DEBUG").is_ok() {
            eprintln!("structure: analyze START");
        }
        let n = body.basic_blocks.len();
        if n == 0 {
            return None;
        }
        let t_all = Instant::now();
        let timing = timing_on();
        if timing {
            eprintln!("[js-timing] analyze START blocks={n}");
        }
        // --- postorder (iterative DFS from the entry block) ---
        let mut po: Vec<usize> = Vec::with_capacity(n);
        let mut po_num = vec![NONE; n];
        let mut on_stack = vec![false; n];
        let mut stack = vec![0usize];
        while let Some(&b) = stack.last() {
            if po_num[b] == NONE && !on_stack[b] {
                on_stack[b] = true;
                for s in successors(body, b) {
                    if po_num[s] == NONE && !on_stack[s] {
                        stack.push(s);
                    }
                }
            } else {
                stack.pop();
                if po_num[b] == NONE {
                    on_stack[b] = false;
                    po_num[b] = po.len();
                    po.push(b);
                }
            }
        }
        if std::env::var("JS_DEBUG").is_ok() {
            eprintln!("structure: postorder done, po_num={:?}", po_num);
        }
        if timing {
            let reach = po_num.iter().filter(|&&p| p != NONE).count();
            eprintln!(
                "[js-timing] analyze postorder DONE reachable={reach}/{n} elapsed={}",
                ms(&t_all)
            );
        }
        // --- dominators over the reachable graph ---
        let mut succs = vec![vec![]; n];
        let mut preds = vec![vec![]; n];
        for b in 0..n {
            for s in successors(body, b) {
                succs[b].push(s);
                preds[s].push(b);
            }
        }
        let mut idom = vec![NONE; n];
        idom[0] = 0;
        let mut order = po.clone();
        order.reverse(); // reverse postorder
        let t_dom = Instant::now();
        let mut changed = true;
        let mut passes = 0usize;
        while changed {
            passes += 1;
            if passes > n + 2 {
                // No convergence: fall back instead of hanging.
                if timing {
                    eprintln!(
                        "[js-timing] analyze dominators NO-CONVERGE passes={passes} elapsed={}",
                        ms(&t_dom)
                    );
                }
                return None;
            }
            changed = false;
            for &b in &order {
                let mut new = NONE;
                for &p in &preds[b] {
                    if idom[p] == NONE {
                        continue;
                    }
                    new = if new == NONE {
                        p
                    } else {
                        let m = intersect(new, p, &idom, &po_num);
                        if m == NONE {
                            break;
                        }
                        m
                    };
                }
                if new != NONE && idom[b] != new {
                    idom[b] = new;
                    changed = true;
                }
            }
        }
        if std::env::var("JS_DEBUG").is_ok() {
            eprintln!("structure: dominators done, idom={:?}", idom);
        }
        if timing {
            eprintln!(
                "[js-timing] analyze dominators DONE passes={passes} elapsed={} total={}",
                ms(&t_dom),
                ms(&t_all)
            );
        }
        // --- natural loops: back edge u -> h means h dominates u ---
        let mut bodies: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut is_header = vec![false; n];
        for &u in &order {
            for &h in &succs[u] {
                if !dominates(h, u, &idom) {
                    continue;
                }
                is_header[h] = true;
                let body = bodies.entry(h).or_default();
                body.insert(h);
                let mut work = vec![u];
                while let Some(x) = work.pop() {
                    if body.insert(x) {
                        for &p in &preds[x] {
                            if p != h {
                                work.push(p);
                            }
                        }
                    }
                }
            }
        }
        // Innermost loop per block: the smallest body containing it.
        let mut header = vec![NONE; n];
        let mut by_size: Vec<(&usize, &HashSet<usize>)> = bodies.iter().collect();
        by_size.sort_by_key(|(_, b)| b.len());
        for (&h, body) in by_size {
            for &b in body.iter() {
                header[b] = h;
            }
        }
        if timing {
            eprintln!(
                "[js-timing] analyze loops DONE headers={} elapsed={}",
                is_header.iter().filter(|&&h| h).count(),
                ms(&t_all)
            );
        }
        if std::env::var("JS_STRUCT_DEBUG").is_ok() && n <= 10 {
            eprintln!("PDOM body n={n} succs={succs:?}");
        }
        let reachable = po_num.iter().map(|&p| p != NONE).collect();
        let _ = &idom;
        // --- post-dominators: CHK over the reversed graph, rooted at a
        // virtual exit node that every returning block flows into. The
        // immediate post-dominator of a branch block is its join.
        let Some(ipdom) = post_dominators(&succs, n) else {
            if timing {
                eprintln!(
                    "[js-timing] analyze ABORT postdoms-failed elapsed={}",
                    ms(&t_all)
                );
            }
            return None;
        };
        if timing {
            eprintln!("[js-timing] analyze DONE blocks={n} elapsed={}", ms(&t_all));
        }
        Some(Structure {
            po_num,
            reachable,
            header,
            is_header,
            bodies,
            ipdom,
        })
    }

    /// Lowest common dominator, or `NONE` when either side is unreachable.
    fn in_body(&self, header: usize, b: usize) -> bool {
        b != VIRTUAL_EXIT && self.bodies.get(&header).is_some_and(|s| s.contains(&b))
    }

    /// Loop-membership query used by the emitter when it renders a loop head.
    pub fn contains_in_loop(&self, header: usize, b: usize) -> bool {
        self.bodies.get(&header).is_some_and(|s| s.contains(&b))
    }

    /// Number of blocks in the body (a size hint for the plan budget).
    pub fn block_count(&self) -> usize {
        self.po_num.len()
    }

    /// How many enclosing loops must be left to reach `b` (0 = `b` is inside
    /// the innermost loop, or there are no loops).
    fn exit_depth(&self, b: usize, loops: &[usize]) -> usize {
        let mut d = 0;
        for &h in loops.iter().rev() {
            if self.in_body(h, b) {
                return d;
            }
            d += 1;
        }
        d
    }

    /// The block a loop header enters at, plus its exit block.
    fn loop_entry(&self, bb: usize, body: &Body<'_>) -> Option<(usize, usize)> {
        let dbg = std::env::var("JS_STRUCT_DEBUG").is_ok();
        let mut exit = NONE;
        if let Some(loop_body) = self.bodies.get(&bb) {
            let mut outs: Vec<usize> = vec![];
            for &b in loop_body.iter() {
                for s in successors(body, b) {
                    if !loop_body.contains(&s) && !outs.contains(&s) {
                        outs.push(s);
                    }
                }
            }
            if outs.len() > 1 {
                if dbg {
                    eprintln!(
                        "LOOP-ENTRY bb={bb} multi-exit outs={outs:?} body_len={}",
                        loop_body.len()
                    );
                }
                return None;
            }
            exit = outs.first().copied().unwrap_or(NONE);
        }
        let term = body.basic_blocks[BasicBlock::from_usize(bb)]
            .terminator
            .as_ref()?;
        if dbg {
            let tname = match &term.kind {
                TerminatorKind::Goto { .. } => "Goto",
                TerminatorKind::SwitchInt { .. } => "SwitchInt",
                TerminatorKind::Call { .. } => "Call",
                TerminatorKind::Drop { .. } => "Drop",
                TerminatorKind::Assert { .. } => "Assert",
                TerminatorKind::Return => "Return",
                _ => "other",
            };
            eprintln!("LOOP-ENTRY bb={bb} term={tname} exit={exit}");
        }
        match &term.kind {
            // Straight-line headers (common after drop elaboration: the
            // back edge lands on a block that calls/drops/asserts before
            // reaching the loop's decision branch). The header effect runs
            // at the top of every iteration; the body (from `first`) holds
            // the decision branch, whose back edge becomes `continue` and
            // whose exit becomes `break`.
            TerminatorKind::Drop { target, .. }
            | TerminatorKind::Call {
                target: Some(target),
                ..
            }
            | TerminatorKind::Assert { target, .. }
            | TerminatorKind::FalseEdge {
                real_target: target,
                ..
            }
            | TerminatorKind::FalseUnwind {
                real_target: target,
                ..
            } => {
                let t = target.index();
                if !self.in_body(bb, t) {
                    if dbg {
                        eprintln!("LOOP-ENTRY bb={bb} straight-line header leaves the loop");
                    }
                    return None;
                }
                Some((t, exit))
            }
            TerminatorKind::Goto { target } => {
                let t = target.index();
                if t == bb {
                    Some((NONE, NONE))
                } else {
                    Some((t, exit))
                }
            }
            TerminatorKind::SwitchInt { targets, .. } => {
                let mut first = NONE;
                let mut distinct = 0;
                for (_, t) in targets.iter() {
                    let t = t.index();
                    if self.in_body(bb, t) {
                        if first == NONE {
                            first = t;
                        } else if first != t {
                            distinct += 1;
                        }
                    }
                }
                if first == NONE || distinct != 0 {
                    return None;
                }
                Some((first, exit))
            }
            _ => None,
        }
    }

    /// Join block of a branch: its immediate post-dominator, i.e. the first
    /// block every path re-converges on.
    fn join_of(&self, bb: usize) -> Option<usize> {
        if bb == NONE || !self.reachable[bb] {
            return None;
        }
        let j = self.ipdom[bb];
        if j == NONE || j == bb {
            return None;
        }
        Some(fold_exit(j, self.po_num.len()))
    }
}

/// Walk `body` and produce a structured plan, or `None` when the shape is not
/// structurally emittable.
pub fn plan(body: &Body<'_>, st: &Structure, budget: usize) -> Option<Vec<Node>> {
    if std::env::var("JS_DEBUG").is_ok() {
        eprintln!("structure: plan START");
    }
    let t_plan = Instant::now();
    let timing = timing_on();
    if timing {
        eprintln!("[js-timing] plan START budget={budget}");
    }
    let reason: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
    let res = plan_inner(body, st, budget, &reason);
    if timing {
        eprintln!(
            "[js-timing] plan DONE ok={} elapsed={}",
            res.is_some(),
            ms(&t_plan)
        );
    }
    if let Some(r) = reason.lock().unwrap().take() {
        if std::env::var("JS_STRUCT_DEBUG").is_ok() {
            eprintln!("STRUCT-FALLBACK: {r}");
        }
    }
    res
}

/// `why` records the first planning failure so the caller can report it.
pub fn plan_inner(
    body: &Body<'_>,
    st: &Structure,
    budget: usize,
    why: &std::sync::Mutex<Option<String>>,
) -> Option<Vec<Node>> {
    let mut planner = Planner {
        st,
        body,
        seen: HashSet::new(),
        budget: budget as isize,
        why,
    };
    let mut out = vec![];
    let mut loops: Vec<usize> = vec![];
    // Exit block per enclosing loop (parallel to `loops`): after `L` comes
    // its exit's code, so `break L` lands exactly there.
    let mut exits: Vec<usize> = vec![];
    planner
        .seq(&mut out, 0, NONE, &mut loops, &mut exits)
        .ok()?;
    Some(out)
}

struct Planner<'w, 'a, 'tb> {
    st: &'a Structure,
    body: &'a Body<'tb>,
    seen: HashSet<usize>,
    budget: isize,
    why: &'w std::sync::Mutex<Option<String>>,
}

impl<'w> Planner<'w, '_, '_> {
    /// Append the plan for the block sequence starting at `bb` (which is not
    /// `stop`) to `out`, following the walk until it reaches `stop`, leaves a
    /// loop, or ends.
    fn seq(
        &mut self,
        out: &mut Vec<Node>,
        mut bb: usize,
        stop: usize,
        loops: &mut Vec<usize>,
        exits: &mut Vec<usize>,
    ) -> Result<(), ()> {
        loop {
            if bb == NONE {
                return Ok(());
            }
            if bb == stop && stop != NONE {
                // Entering the stop block: a back edge when it is an
                // enclosing header, otherwise the enclosing branch joins
                // there and this path is over.
                if let Some(pos) = loops.iter().rposition(|&h| h == bb) {
                    out.push(Node::Jump {
                        depth: loops.len() - pos,
                        is_break: false,
                    });
                }
                return Ok(());
            }
            if !self.seen.insert(bb) {
                // Shared-tail diamond: two arms converge before the computed
                // join (some path avoids the early convergence, so it is not
                // the post-dominator). Re-emit the tail block in this arm
                // instead of bailing: each copy sits in a disjoint runtime
                // path, so this is path inlining, and the per-visit budget
                // below caps total work (overrun falls back, never hangs).
                // Loop headers are exempt: duplicating a loop would break
                // back-edge nesting, so those still fall back.
                if self.st.is_header[bb] || loops.contains(&bb) {
                    self.bail(&format!("block {bb} is reachable on two paths"))?;
                    return Err(());
                }
            }
            self.budget -= 1;
            if self.budget < 0 {
                self.bail("budget exceeded")?;
                return Err(());
            }
            // A loop header starts a loop (the header itself is the stop
            // block of its own body, so it is never re-entered here).
            if self.st.is_header[bb] {
                let (first, exit) = self.st.loop_entry(bb, self.body).ok_or_else(|| {
                    let _ = self.bail("a loop has more than one exit or an odd entry");
                    ()
                })?;
                loops.push(bb);
                exits.push(exit);
                let mut body = vec![];
                let res = self.seq(&mut body, first, bb, loops, exits);
                exits.pop();
                loops.pop();
                res?;
                out.push(Node::Loop { bb, body });
                if exit == NONE {
                    out.push(Node::Stop);
                    return Ok(());
                }
                bb = exit;
                continue;
            }
            let bt = self.body.basic_blocks[BasicBlock::from_usize(bb)]
                .terminator
                .as_ref()
                .ok_or_else(|| {
                    let _ = self.bail("a block has no terminator");
                    ()
                })?;
            match &bt.kind {
                TerminatorKind::Goto { target } => {
                    out.push(Node::Block(bb));
                    bb = self.next_ok(out, target.index(), stop, loops)?;
                    if bb == NONE {
                        return Ok(());
                    }
                }
                TerminatorKind::SwitchInt { targets, .. } => {
                    let join = match self.st.join_of(bb) {
                        Some(j) => j,
                        None => {
                            self.bail(&format!(
                                "no join for block {bb} in {}",
                                self.path_trace(bb)
                            ))?;
                            return Err(());
                        }
                    };
                    if std::env::var("JS_STRUCT_DEBUG").is_ok() {
                        let ts: Vec<usize> = targets.iter().map(|(_, t)| t.index()).collect();
                        eprintln!(
                            "PLAN bb={bb} join={join} succs={:?} otherwise={} stop={stop} loops={loops:?} depth={}",
                            ts,
                            targets.otherwise().index(),
                            self.st.exit_depth(join, loops)
                        );
                    }
                    // The merge has to stay inside the loops we are in,
                    // unless it is the function's exit (every arm returns).
                    // An outside merge is still structural when every path
                    // that falls through to it lands correctly: the arm
                    // walks end there silently (or left via `break` /
                    // `continue` / `return` on their own), and the
                    // continuation below is exact in exactly three cases --
                    // the merge is where the enclosing list continues
                    // (`stop`), it is an enclosing loop header (`continue`
                    // re-runs the header, which is what reaching it means),
                    // or it is the exit of an enclosing loop (`break`
                    // lands at the start of that exit's code).
                    if join != VIRTUAL_EXIT && self.st.exit_depth(join, loops) != 0 {
                        let d = self.st.exit_depth(join, loops);
                        let at_loop_exit = exits.get(loops.len() - d).is_some_and(|&e| e == join);
                        if !(join == stop || loops.contains(&join) || at_loop_exit) {
                            self.bail("a join is outside the enclosing loop")?;
                            return Err(());
                        }
                    }
                    let mut arms = vec![];
                    for (v, t) in targets.iter() {
                        let mut nodes = vec![];
                        self.seq(&mut nodes, t.index(), join, loops, exits)?;
                        arms.push((v, nodes));
                    }
                    let mut otherwise = vec![];
                    self.seq(
                        &mut otherwise,
                        targets.otherwise().index(),
                        join,
                        loops,
                        exits,
                    )?;
                    out.push(Node::Branch {
                        bb,
                        arms,
                        otherwise,
                    });
                    if join == VIRTUAL_EXIT {
                        out.push(Node::Stop);
                        return Ok(());
                    }
                    bb = self.next_ok(out, join, stop, loops)?;
                    if bb == NONE {
                        return Ok(());
                    }
                }
                TerminatorKind::Drop { target, .. }
                | TerminatorKind::Call {
                    target: Some(target),
                    ..
                }
                | TerminatorKind::Assert { target, .. }
                | TerminatorKind::FalseEdge {
                    real_target: target,
                    ..
                }
                | TerminatorKind::FalseUnwind {
                    real_target: target,
                    ..
                } => {
                    out.push(Node::Block(bb));
                    bb = self.next_ok(out, target.index(), stop, loops)?;
                    if bb == NONE {
                        return Ok(());
                    }
                }
                TerminatorKind::Return
                | TerminatorKind::UnwindResume
                | TerminatorKind::UnwindTerminate(_)
                | TerminatorKind::Unreachable
                | TerminatorKind::TailCall { .. } => {
                    out.push(Node::Block(bb));
                    out.push(Node::Stop);
                    return Ok(());
                }
                TerminatorKind::Call { target: None, .. } => {
                    out.push(Node::Block(bb));
                    out.push(Node::Stop);
                    return Ok(());
                }
                _ => {
                    self.bail("an unmappable terminator")?;
                    return Err(());
                }
            }
        }
    }

    /// The walk path leading to `bb`, for diagnostics.
    fn path_trace(&self, bb: usize) -> String {
        let mut v: Vec<usize> = self.seen.iter().copied().collect();
        v.sort_unstable();
        v.push(bb);
        v.iter()
            .take(12)
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Record why planning bailed out (for `JS_STRUCT_DEBUG`).
    fn bail(&self, why: &str) -> Result<(), ()> {
        *self.why.lock().unwrap() = Some(why.to_string());
        Err(())
    }

    /// Decide what `next` means control-flow-wise and emit the jump for it.
    /// Returns the block to keep walking at, or `NONE` when this path ends.
    fn next_ok(
        &mut self,
        out: &mut Vec<Node>,
        next: usize,
        stop: usize,
        loops: &[usize],
    ) -> Result<usize, ()> {
        // A branch join or a loop's back edge: the enclosing list continues.
        if next == stop && stop != NONE {
            if let Some(pos) = loops.iter().rposition(|&h| h == next) {
                out.push(Node::Jump {
                    depth: loops.len() - pos,
                    is_break: false,
                });
            }
            return Ok(NONE);
        }
        // A back edge to any enclosing loop.
        if let Some(pos) = loops
            .iter()
            .rposition(|h| self.st.is_header[next] && *h == next)
        {
            out.push(Node::Jump {
                depth: loops.len() - pos,
                is_break: false,
            });
            return Ok(NONE);
        }
        // Leaving the loops we are in.
        let d = self.st.exit_depth(next, loops);
        if d != 0 {
            out.push(Node::Jump {
                depth: d,
                is_break: true,
            });
            return Ok(NONE);
        }
        Ok(next)
    }
}
