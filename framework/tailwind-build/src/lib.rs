//! Splits the complete Tailwind output (all classes, plugins included) into
//! per-page critical CSS (inlined into the prerendered HTML) and the lazy
//! remainder (served as a stylesheet and applied after load).

use lightningcss::rules::CssRule;
use lightningcss::selector::{Component, Selector};
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use std::collections::HashSet;

pub struct Split {
    /// Classless rules (preflight, `@font-face`, keyframes, ...) plus every
    /// rule whose selectors only reference classes present in the page.
    pub critical: String,
    /// Everything else, minified.
    pub lazy: String,
}

/// Collects every whitespace-separated token of every `class="..."` /
/// `class='...'` attribute in the markup.
pub fn extract_classes(html: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let bytes = html.as_bytes();
    let mut i = 0;
    while let Some(pos) = html[i..].find("class=") {
        let mut j = i + pos + 6;
        if j < bytes.len() && (bytes[j] == b'"' || bytes[j] == b'\'') {
            let quote = bytes[j];
            j += 1;
            let start = j;
            while j < bytes.len() && bytes[j] != quote {
                j += 1;
            }
            for tok in html[start..j].split_whitespace() {
                out.insert(tok.to_string());
            }
            i = j + 1;
        } else {
            i = j;
        }
    }
    out
}

/// Union of [`extract_classes`] over all markers given.
pub fn extract_classes_many(htmls: &[&str]) -> HashSet<String> {
    let mut out = HashSet::new();
    for html in htmls {
        out.extend(extract_classes(html));
    }
    out
}

fn selector_used(sel: &Selector, used: &HashSet<String>) -> bool {
    sel.iter_raw_match_order().all(|comp| match comp {
        Component::Class(c) => used.contains(c.0.as_ref()),
        _ => true,
    })
}

fn selector_has_class(sel: &Selector) -> bool {
    sel.iter_raw_match_order()
        .any(|comp| matches!(comp, Component::Class(_)))
}

fn selector_only_in(sel: &Selector, used: &HashSet<String>) -> bool {
    selector_has_class(sel) && selector_used(sel, used)
}

fn split_rules<'i>(
    rules: Vec<CssRule<'i>>,
    used: &HashSet<String>,
    crit: &mut Vec<CssRule<'i>>,
    lazy: &mut Vec<CssRule<'i>>,
) {
    for rule in rules {
        match rule {
            CssRule::Style(mut s) => {
                let mut crit_rule = s.clone();
                crit_rule.selectors.0.retain(|sel| selector_used(sel, used));
                if !crit_rule.selectors.0.is_empty() {
                    crit.push(CssRule::Style(crit_rule));
                }
                s.selectors.0.retain(|sel| !selector_used(sel, used));
                if !s.selectors.0.is_empty() {
                    lazy.push(CssRule::Style(s));
                }
            }
            CssRule::Media(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules(inner, used, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::Media(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lazy.push(CssRule::Media(m));
                }
            }
            CssRule::Supports(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules(inner, used, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::Supports(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lazy.push(CssRule::Supports(m));
                }
            }
            CssRule::LayerBlock(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules(inner, used, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::LayerBlock(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lazy.push(CssRule::LayerBlock(m));
                }
            }
            // Classless at-rules (preflight, @font-face, @keyframes, imports,
            // @property, unknown at-rules, ...) always belong to the critical
            // slice; they cannot be attributed to a page class.
            other => crit.push(other),
        }
    }
}

fn serialize<'i>(mut sheet: StyleSheet<'i>, rules: Vec<CssRule<'i>>) -> String {
    sheet.rules.0 = rules;
    sheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .expect("serialize CSS")
        .code
}

fn split_rules_sets<'i>(
    rules: Vec<CssRule<'i>>,
    critical: &HashSet<String>,
    lazy: &HashSet<String>,
    crit: &mut Vec<CssRule<'i>>,
    lz: &mut Vec<CssRule<'i>>,
) {
    for rule in rules {
        match rule {
            CssRule::Style(mut s) => {
                let mut crit_rule = s.clone();
                crit_rule.selectors.0.retain(|sel| selector_used(sel, critical));
                if !crit_rule.selectors.0.is_empty() {
                    crit.push(CssRule::Style(crit_rule));
                }
                s.selectors.0.retain(|sel| selector_only_in(sel, lazy));
                if !s.selectors.0.is_empty() {
                    lz.push(CssRule::Style(s));
                }
            }
            CssRule::Media(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules_sets(inner, critical, lazy, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::Media(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lz.push(CssRule::Media(m));
                }
            }
            CssRule::Supports(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules_sets(inner, critical, lazy, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::Supports(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lz.push(CssRule::Supports(m));
                }
            }
            CssRule::LayerBlock(mut m) => {
                let inner = std::mem::take(&mut m.rules.0);
                let mut c = Vec::new();
                let mut l = Vec::new();
                split_rules_sets(inner, critical, lazy, &mut c, &mut l);
                if !c.is_empty() {
                    let mut cm = m.clone();
                    cm.rules.0 = c;
                    crit.push(CssRule::LayerBlock(cm));
                }
                if !l.is_empty() {
                    m.rules.0 = l;
                    lz.push(CssRule::LayerBlock(m));
                }
            }
            other => crit.push(other),
        }
    }
}

/// Splits into rules for `critical` classes and rules for `lazy` classes,
/// dropping every rule that matches neither.
pub fn split_sets(
    full_css: &str,
    critical: &HashSet<String>,
    lazy: &HashSet<String>,
) -> Result<Split, String> {
    let sheet = StyleSheet::parse(full_css, ParserOptions::default())
        .map_err(|e| format!("parse tailwind CSS: {e}"))?;
    let mut crit = Vec::new();
    let mut lz = Vec::new();
    split_rules_sets(sheet.rules.0.clone(), critical, lazy, &mut crit, &mut lz);
    Ok(Split {
        critical: serialize(sheet.clone(), crit),
        lazy: serialize(sheet, lz),
    })
}

pub fn split(full_css: &str, used: &HashSet<String>) -> Result<Split, String> {
    let sheet = StyleSheet::parse(full_css, ParserOptions::default())
        .map_err(|e| format!("parse tailwind CSS: {e}"))?;
    let mut crit = Vec::new();
    let mut lazy = Vec::new();
    split_rules(sheet.rules.0.clone(), used, &mut crit, &mut lazy);
    Ok(Split {
        critical: serialize(sheet.clone(), crit),
        lazy: serialize(sheet, lazy),
    })
}

/// Convenience: union the classes of all pages, then [`split`].
pub fn split_for_pages(full_css: &str, pages: &[&str]) -> Result<Split, String> {
    let used = extract_classes_many(pages);
    split(full_css, &used)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSS: &str = "*,::before{--x:0} .p-4{padding:1rem} .mb-4{margin:1rem} @media (min-width:640px){.p-4{padding:2rem}.mt-4{margin:1rem}}";

    #[test]
    fn extracts_classes() {
        let mut used: Vec<String> = extract_classes(
            r#"<div class="p-4  mb-4" class='mt-2'></div>"#,
        )
        .into_iter()
        .collect();
        used.sort();
        assert_eq!(used, ["mb-4", "mt-2", "p-4"]);
    }

    #[test]
    fn splits_used_from_unused() {
        let split = split(CSS, &["p-4".to_string()].into_iter().collect()).unwrap();
        assert!(split.critical.contains("--x:0"));
        assert!(split.critical.contains("padding:1rem"));
        assert!(!split.lazy.contains("padding:1rem"));
        assert!(split.lazy.contains("margin:1rem"));
        assert!(split.critical.contains("@media"));
        assert!(split.critical.contains("padding:2rem"));
        assert!(split.lazy.contains("@media"));
    }

    #[test]
    fn classless_rules_are_always_critical() {
        let split = split(CSS, &HashSet::new()).unwrap();
        assert!(split.critical.contains("--x:0"));
        assert!(!split.lazy.contains("--x:0"));
        assert!(!split.critical.contains("padding"));
        assert!(split.lazy.contains("padding:1rem"));
    }
}
