#![no_std]
#![allow(dead_code)]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Debug, Default)]
pub struct StyleRegistry {
    classes: Vec<String>,
}

impl StyleRegistry {
    pub fn new() -> Self {
        Self { classes: Vec::new() }
    }

    pub fn load(classes: &[&str]) -> Self {
        let mut r = Self::new();
        for c in classes {
            r.add(*c);
        }
        r
    }

    pub fn load_str(input: &str) -> Self {
        Self::load_vec(input.split_whitespace().map(str::to_string).collect())
    }

    fn load_vec(classes: Vec<String>) -> Self {
        let mut r = Self::new();
        for c in classes {
            r.add(c);
        }
        r
    }

    pub fn add(&mut self, class: impl Into<String>) {
        let s = class.into();
        if !s.trim().is_empty() && !self.classes.iter().any(|x| x == &s) {
            self.classes.push(s);
        }
    }

    pub fn add_many(&mut self, classes: impl IntoIterator<Item = impl Into<String>>) {
        for c in classes {
            self.add(c);
        }
    }

    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    pub fn joined(&self) -> String {
        self.classes.join(" ")
    }

    pub fn unique(&self) -> Vec<String> {
        let mut out = Vec::new();
        for c in &self.classes {
            if !out.iter().any(|x: &String| x == c) {
                out.push(c.clone());
            }
        }
        out
    }

    pub fn to_class_attr(&self) -> String {
        alloc::format!("class=\"{}\"", self.joined())
    }

    pub fn to_manifest_json(&self) -> String {
        let mut s = String::from("[");
        for (i, c) in self.classes.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push('"');
            s.push_str(c);
            s.push('"');
        }
        s.push(']');
        s
    }

    pub fn inject_snippet(&self) -> String {
        let classes = self.joined();
        alloc::format!("fwTailwindInject(document.body, \"{}\");", classes)
    }

    pub fn injector_js() -> &'static str {
        INJECTOR_JS
    }
}

#[derive(Clone, Debug, Default)]
pub struct DynamicStyleRegistry {
    classes: Vec<String>,
}

impl DynamicStyleRegistry {
    pub fn new() -> Self {
        Self { classes: Vec::new() }
    }

    pub fn load(&mut self, class: impl Into<String>) {
        self.classes.push(class.into());
    }

    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    pub fn joined(&self) -> String {
        self.classes.join(" ")
    }

    pub fn inject_js(&self) -> String {
        let classes = self.joined();
        alloc::format!(
            "fwTailwindLoadStyle({:?}); fwTailwindInjectAll(\"[data-tw-dynamic]\", \"{}\");",
            classes,
            classes
        )
    }
}

pub const INJECTOR_JS: &str = r#"
export function fwTailwindInject(el, classes) {
  if (!el) return;
  const list = Array.isArray(classes) ? classes : String(classes).split(/\s+/).filter(Boolean);
  for (const c of list) el.classList.add(c);
}
export function fwTailwindInjectAll(selector, classes) {
  document.querySelectorAll(selector).forEach((el) => fwTailwindInject(el, classes));
}
export function fwTailwindLoadStyle(css) {
  const style = document.createElement("style");
  style.textContent = css;
  document.head.appendChild(style);
}
"#;

pub fn load_classes(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::to_string).collect()
}

#[macro_export]
macro_rules! tw {
    ($s:expr) => {
        $s
    };
}

#[macro_export]
macro_rules! style_registry {
    ($($c:expr),* $(,)?) => {{
        let mut r = $crate::StyleRegistry::new();
        $( r.add($c); )*
        r
    }};
}

#[macro_export]
macro_rules! classes {
    ($($c:expr),* $(,)?) => {
        {
            let mut v = ::alloc::vec::Vec::new();
            $( v.push(::alloc::string::ToString::to_string($c)); )*
            v
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut r = StyleRegistry::new();
        r.add("flex");
        r.add("p-4");
        r.add("flex");
        assert_eq!(r.classes(), &["flex", "p-4"]);
        assert_eq!(r.joined(), "flex p-4");
    }

    #[test]
    fn load() {
        let r = StyleRegistry::load(&["a", "b"]);
        assert_eq!(r.joined(), "a b");
    }

    #[test]
    fn macro_works() {
        let r = style_registry!("x", "y");
        assert_eq!(r.joined(), "x y");
    }

    #[test]
    fn manifest() {
        let r = style_registry!("a", "b");
        assert_eq!(r.to_manifest_json(), "[\"a\",\"b\"]");
    }
}
