#![feature(register_tool)]
#![register_tool(js)]
#![allow(unused_must_use, dead_code)]

use std::fmt::Write as _;

#[js::export]
pub fn run() -> String {
    let mut s = String::new();
    // construction / push / pop / indexing
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    s.push_str(&format!(
        "{} {} {} {} ",
        v.len(),
        v[0],
        v[3],
        v[v.len() - 1]
    ));
    s.push_str(&format!("{:?} ", v));
    s.push_str(&format!("{:?} ", v.pop()));
    s.push_str(&format!("{:?} {:?} ", v.first(), v.last()));
    s.push_str(&format!("{:?} ", v.get(1)));
    s.push_str(&format!("{:?} ", v.get(9)));
    v.insert(1, 99);
    v.remove(0);
    s.push_str(&format!("{:?} ", v));
    v.clear();
    v.extend_from_slice(&[7, 8, 9]);
    s.push_str(&format!("{:?} ", v));
    let w: Vec<i32> = vec![10; 3];
    s.push_str(&format!("{:?} ", w));
    let mut u = vec![1, 2, 3, 4, 5];
    u.truncate(2);
    s.push_str(&format!("{:?} ", u));
    u.retain(|x| *x == 1);
    s.push_str(&format!("{:?} ", u));
    let mut z = vec![5, 1, 4, 2, 3];
    z.sort();
    s.push_str(&format!("{:?} ", z));
    z.reverse();
    s.push_str(&format!("{:?} ", z));
    // out of bounds panics (unwrapped later in panic corpus)
    // let _ = v[100];
    // slices
    let sl: &[i32] = &z;
    s.push_str(&format!(
        "{} {} {:?} {:?} ",
        sl.len(),
        sl[0],
        sl.first(),
        sl.last()
    ));
    let m = &sl[1..3];
    s.push_str(&format!("{:?} ", m));
    let mut sum = 0i32;
    for x in m {
        sum += *x;
    }
    s.push_str(&format!("{} ", sum));
    s.push_str(&format!("{:?} ", m.contains(&3)));
    s.push_str(&format!("{:?} ", m.contains(&99)));
    s.push_str(&format!("{:?} ", m.starts_with(&[3])));
    s.push_str(&format!("{:?} ", m.ends_with(&[3])));
    let mut cv = m.to_vec();
    cv.push(100);
    s.push_str(&format!("{:?} ", cv));
    // Option / Result
    let a: Option<i32> = Some(5);
    let b: Option<i32> = None;
    s.push_str(&format!("{:?} {:?} ", a, b));
    s.push_str(&format!("{} ", a.is_some()));
    s.push_str(&format!("{} ", a.is_none()));
    s.push_str(&format!("{:?} ", a.map(|x| x * 2)));
    s.push_str(&format!(
        "{:?} ",
        a.and_then(|x| if x > 0 { Some(-x) } else { None })
    ));
    s.push_str(&format!("{:?} ", a.unwrap_or(0) + b.unwrap_or(7)));
    s.push_str(&format!("{:?} ", a.ok_or("err")));
    s.push_str(&format!("{:?} ", b.ok_or("err")));
    s.push_str(&format!("{:?} ", a.filter(|x| *x > 10)));
    let r: Result<i32, &str> = Ok(1);
    let r2: Result<i32, &str> = Err("bad");
    s.push_str(&format!("{:?} {:?} ", r, r2));
    s.push_str(&format!("{} ", r.is_ok()));
    s.push_str(&format!("{:?} ", r.map(|x| x + 1)));
    s.push_str(&format!("{:?} ", r.and_then(|x| Ok(x * 3))));
    s.push_str(&format!("{:?} ", r.err()));
    s.push_str(&format!("{:?} ", r2.unwrap_or_else(|e| e.len() as i32)));
    s.push_str(&format!("{:?} ", r2.as_ref().err().copied()));
    // iterator chains
    let it: Vec<i32> = (1..=5).map(|x| x * x).filter(|x| *x % 2 == 1).collect();
    s.push_str(&format!("{:?} ", it));
    let f: i32 = (1..=4).fold(0, |a, x| a + x);
    s.push_str(&format!("{} ", f));
    s.push_str(&format!("{} ", it.iter().count()));
    s.push_str(&format!("{} ", it.iter().sum::<i32>()));
    s.push_str(&format!("{} ", it.iter().rev().min().unwrap()));
    s.push_str(&format!("{} ", it.iter().max().unwrap()));
    s.push_str(&format!("{} ", it.iter().any(|x| *x == 9)));
    s.push_str(&format!("{} ", it.iter().all(|x| *x > 0)));
    s.push_str(&format!("{} ", it.iter().position(|x| *x == 9).unwrap()));
    // collect options/results
    let opts: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let flat: Vec<i32> = opts.into_iter().flatten().collect();
    s.push_str(&format!("{:?} ", flat));
    let joined = it
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    s.push_str(&format!("{} ", joined));
    // vec of tuples / nested
    let pairs: Vec<(i32, i32)> = vec![(1, 2), (3, 4)];
    s.push_str(&format!("{:?} ", pairs));
    s.push_str(&format!(
        "{:?} ",
        pairs.iter().map(|(a, b)| a + b).collect::<Vec<_>>()
    ));
    // Box
    let bx = Box::new(vec![1, 2, 3]);
    s.push_str(&format!("{:?} ", bx.as_ref()));
    let bx2 = bx.clone();
    s.push_str(&format!("{:?} ", bx2));
    // Cell / RefCell
    let c = std::cell::Cell::new(5i32);
    c.set(6);
    s.push_str(&format!("{} ", c.get()));
    let rc = std::cell::RefCell::new(vec![1, 2]);
    rc.borrow_mut().push(3);
    s.push_str(&format!("{:?} ", rc.borrow()));
    // write! into string
    write!(s, "{} {} ", 42, "x");
    s
}
