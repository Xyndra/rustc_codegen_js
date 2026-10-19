#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub type ComponentId = u64;

/// Sparse id-keyed map backed by a `Vec`. `alloc::collections::BTreeMap`
/// monomorphizes into the `alloc` crate, whose functions are not compiled
/// by the JS backend (no `alloc.mjs` bundle exists), so cross-crate calls
/// would dangle. Linear scans are fine at scheduler/registry sizes.
#[derive(Clone, Debug, Default)]
pub struct ComponentMap<V> {
    entries: Vec<(ComponentId, V)>,
}

impl<V> ComponentMap<V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: ComponentId, value: V) -> Option<V> {
        let mut i = 0;
        while i < self.entries.len() {
            if self.entries[i].0 == id {
                return Some(core::mem::replace(&mut self.entries[i].1, value));
            }
            i += 1;
        }
        self.entries.push((id, value));
        None
    }

    pub fn get(&self, id: ComponentId) -> Option<&V> {
        let mut i = 0;
        while i < self.entries.len() {
            if self.entries[i].0 == id {
                return Some(&self.entries[i].1);
            }
            i += 1;
        }
        None
    }

    pub fn get_mut(&mut self, id: ComponentId) -> Option<&mut V> {
        let mut i = 0;
        while i < self.entries.len() {
            if self.entries[i].0 == id {
                return Some(&mut self.entries[i].1);
            }
            i += 1;
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirtyMask {
    None,
    SelfOnly,
    Subtree,
}

pub struct ComponentState {
    pub id: ComponentId,
    pub dirty: DirtyMask,
    pub parent: Option<ComponentId>,
    pub children: Vec<ComponentId>,
}

pub struct RenderCx<'a> {
    pub scheduler: &'a mut RenderScheduler,
}

pub struct RenderScheduler {
    pub pending: Vec<ComponentId>,
    pub rendering: bool,
    next_id: u64,
    pub rendered: ComponentMap<String>,
}

impl RenderScheduler {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            rendering: false,
            next_id: 1,
            rendered: ComponentMap::new(),
        }
    }

    pub fn allocate_id(&mut self) -> ComponentId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn schedule(&mut self, id: ComponentId) {
        if !self.pending.contains(&id) {
            self.pending.push(id);
        }
    }

    pub fn is_pending(&self, id: ComponentId) -> bool {
        self.pending.contains(&id)
    }

    pub fn rendered_string(&self, id: ComponentId) -> Option<&String> {
        self.rendered.get(id)
    }

    pub fn set_rendered(&mut self, id: ComponentId, value: String) {
        self.rendered.insert(id, value);
    }

    pub fn end_render(&mut self) {
        if !self.rendering {
            self.pending.clear();
        }
    }

    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    pub fn drain_pending(&mut self) -> Vec<ComponentId> {
        core::mem::take(&mut self.pending)
    }
}

pub struct ComponentRegistry {
    pub states: ComponentMap<DirtyMask>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self { states: ComponentMap::new() }
    }

    pub fn register(&mut self, id: ComponentId) {
        self.states.insert(id, DirtyMask::None);
    }

    pub fn mark(&mut self, id: ComponentId, mask: DirtyMask) {
        self.states.insert(id, mask);
    }

    pub fn get(&self, id: ComponentId) -> DirtyMask {
        self.states.get(id).copied().unwrap_or(DirtyMask::None)
    }

    pub fn clear(&mut self, id: ComponentId) {
        self.states.insert(id, DirtyMask::None);
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Component {
    fn id(&self) -> ComponentId;
    fn set_dirty(&mut self, mask: DirtyMask);
    fn is_dirty(&self) -> DirtyMask;

    fn render(&mut self, cx: &mut RenderCx) -> String;

    fn possible_css_classes(&self) -> &'static str {
        ""
    }

    fn mark_dirty(&mut self) {
        self.set_dirty(DirtyMask::SelfOnly);
    }

    fn mark_subtree_dirty(&mut self) {
        self.set_dirty(DirtyMask::Subtree);
    }
}

pub type Setter<T> = alloc::boxed::Box<dyn FnMut(T)>;

pub mod prelude {
    pub use super::*;
}
