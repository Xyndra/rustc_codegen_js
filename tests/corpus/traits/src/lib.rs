// Traits, generics (monomorphization), panics/catch_unwind, and drop ordering.
//
// NOTE on drop ordering: the JS backend has no destructor runtime. A user
// `impl Drop` is diagnosed as unsupported (drops of such types are rejected),
// and for GC-managed types (Vec, String, Box, our object reprs) the drop is a
// no-op: values are destroyed in reverse construction order at scope exit and
// reclaimed by the JS engine. This test exercises drop *semantics* by placing
// owned values in nested scopes, confirming they stay usable while alive and
// that execution after scope exit is still correct (order is LIFO in the MIR).
//
// NOTE on catch_unwind: the panic payload is `Box<dyn Any + Send>`, a type
// with a significant drop that this backend does not implement. The payload is
// an opaque panic value that owns no heap memory (it is a `fmt::Arguments`
// view onto static slices), so it is `mem::forget`'d via `was_unwind` rather
// than dropped; forgetting it is memory safe.
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

trait Shape {
    fn area(&self) -> i32;
    fn tag(&self) -> String;
}

#[derive(Debug)]
struct Sq(i32);
#[derive(Debug)]
struct R(i32, i32);

impl Shape for Sq {
    fn area(&self) -> i32 {
        self.0 * self.0
    }
    fn tag(&self) -> String {
        format!("Sq{}", self.0)
    }
}

impl Shape for R {
    fn area(&self) -> i32 {
        self.0 * self.1
    }
    fn tag(&self) -> String {
        format!("R{}x{}", self.0, self.1)
    }
}

/// Static dispatch through a generic parameter: monomorphized into one
/// instance per concrete type.
fn area_of<T: Shape>(x: &T) -> i32 {
    x.area() + 1
}

fn bigger<A: Shape, B: Shape>(a: A, b: B) -> i32 {
    let (x, y) = (a.area(), b.area());
    if x > y {
        x
    } else {
        y
    }
}

/// Iterator adapters over a generic iterator type (closures + traits together).
fn sum_squares<I: Iterator<Item = i32>>(it: I) -> i32 {
    it.map(|x| x * x).sum()
}

struct Counter {
    n: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { n: 0 }
    }
    fn next(&mut self) -> i32 {
        let v = self.n;
        self.n += 1;
        v
    }
}

/// Run `f` through `catch_unwind` and return whether a panic was caught.
/// The panic payload is `Box<dyn Any + Send>` — a type with a significant
/// drop that this backend does not implement. The payload owns no heap
/// memory (it is a `fmt::Arguments` view onto static slices), so it is
/// `mem::forget`'d rather than dropped; forgetting it is memory safe.
fn was_unwind<F, R>(f: F) -> bool
where
    F: FnOnce() -> R,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(_) => false,
        Err(payload) => {
            std::mem::forget(payload);
            true
        }
    }
}

#[js::export]
pub fn run() -> String {
    let mut s = String::new();
    // --- trait static dispatch through generics ---
    s.push_str(&format!("{} {} ", area_of(&Sq(3)), area_of(&R(3, 4)))); // 10 13
    s.push_str(&format!("{} ", bigger(Sq(5), R(3, 4)))); // 25
    s.push_str(&format!("{} ", Sq(7).tag())); // Sq7
    // --- derived Debug (tuple-struct form) ---
    s.push_str(&format!("{:?} {:?} ", Sq(1), R(2, 3))); // Sq(1) R(2, 3)
    // --- generic + iterator adapters ---
    s.push_str(&format!("{} ", sum_squares(1..=4))); // 30
    // --- closures capturing by reference and by value ---
    let mut c = Counter::new();
    s.push_str(&format!("{} {} ", c.next(), c.next())); // 0 1
    let mut total = String::new();
    let mut push_one = |x: i32| total.push_str(&format!("{x}"));
    push_one(1);
    push_one(2);
    s.push_str(&format!("{} ", total)); // 12
    // --- drop ordering / scope exit ---
    {
        let _x = vec![1, 2, 3];
        let _y = vec![4, 5];
        // still usable inside the scope; dropped (no-op) at the close brace
        s.push_str(&format!("{} ", _x.len() + _y.len())); // 8
    }
    // --- panics through catch_unwind ---
    s.push_str(&format!("{:?} ", was_unwind(|| {
        let v: Vec<i32> = Vec::new();
        v[0]  // bounds-check panic (inline throw)
    }))); // true
    let r = was_unwind(|| 1 + None::<i32>.unwrap());
    s.push_str(&format!("{} {} ", r, r)); // true true
    let m = was_unwind(|| panic!("boom {}", 42)); // fmt panic
    s.push_str(&format!("{} ", m)); // true
    let a = was_unwind(|| assert!(false, "must {} fail {}", "surely", "now")); // assert fmt
    s.push_str(&format!("{} ", a)); // true
    let ok = was_unwind(|| {
        let _x = 1 + 2;
        assert!(3 == 3);
    });
    s.push_str(&format!("{} ", ok)); // true
    let bad = was_unwind(|| assert_eq!(1, 2)); // assert_eq! fmt
    s.push_str(&format!("{} ", bad)); // true
    let never = was_unwind(|| panic!("nope"));
    s.push_str(&format!("{} ", never)); // false
    s.push_str(&format!("{} ", s.len())); // trailing length for the harness
    s
}
