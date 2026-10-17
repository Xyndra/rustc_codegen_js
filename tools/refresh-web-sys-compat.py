import pathlib
import re

TYPE_MAP = {}


def parse_macro_attr(line):
    m = re.search(r"wasm_bindgen\s*\((.*)\)", line)
    args = m.group(1) if m else ""
    res = {}
    for part in args.split(","):
        part = part.strip()
        if "=" in part:
            k, v = part.split("=", 1)
            res[k.strip()] = v.strip()
        elif part:
            res[part] = True
    return res


def stripq(x):
    if isinstance(x, str):
        x = x.strip()
        if x.startswith('"') and x.endswith('"'):
            return x[1:-1]
    return x


def split_attrs(attr_lines):
    macro = []
    other = []
    for l in attr_lines:
        if "wasm_bindgen" in l and l.strip().startswith("#"):
            macro.append(l)
        else:
            other.append(l)
    mm = parse_macro_attr("\n".join(macro)) if macro else {}
    return mm, other


def extra_params(params_str):
    p = params_str.strip()
    if not p:
        return []
    depth = 0
    cur = ""
    parts = []
    for ch in p:
        if ch in "<(":
            depth += 1
            cur += ch
        elif ch in ">)":
            depth = max(0, depth - 1)
            cur += ch
        elif ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return [x.strip() for x in parts if x.strip()]


def first_param_name(params_str):
    ps = extra_params(params_str)
    if not ps:
        return None
    m = re.match(r"(\w+)\s*:", ps[0])
    return m.group(1) if m else None


def remove_first_param(params_str):
    ps = extra_params(params_str)
    if ps and re.match(r"this\s*:", ps[0]):
        ps = ps[1:]
    return ", ".join(ps)


def get_js(attrs):
    return stripq(attrs.get("js_name")) or stripq(attrs.get("js_class"))


def transform_type(macro, other, name):
    key = stripq(macro.get("js_name")) or stripq(macro.get("js_class"))
    if key:
        TYPE_MAP[key] = name
    lines = other + [
        "#[js::opaque]",
        "#[derive(Copy)]",
        f"pub struct {name}(pub ::wasm_bindgen::JsValue);",
    ]
    return "\n".join(lines)


def build_import(attrs, params_str):
    jn = stripq(attrs.get("js_name"))
    namespace = stripq(attrs.get("js_namespace"))
    static_of = stripq(attrs.get("static_method_of"))
    js_class = stripq(attrs.get("js_class"))
    getter = "getter" in attrs
    setter = "setter" in attrs
    constructor = "constructor" in attrs
    has_this = (
        (js_class is not None or static_of is not None)
        and first_param_name(params_str) == "this"
        and "static_method_of" not in attrs
        and not constructor
    )
    # in web-sys method attr has 'method' key
    if js_class is None and static_of is None:
        # free/namespace function; no this
        has_this = False

    if has_this:
        rest = extra_params(params_str)[1:]
        n = len(rest)
    else:
        rest = extra_params(params_str)
        n = len(rest)

    if getter:
        if js_class:
            path = f"$0.{jn}"
        elif namespace:
            path = f"{namespace}.{jn}"
        else:
            path = jn
    elif setter:
        if js_class:
            path = f"$0.{jn} = $1"
        elif namespace:
            path = f"{namespace}.{jn} = $1"
        else:
            path = f"{jn} = $1"
    elif constructor:
        cls = js_class or static_of or jn
        if n == 0:
            path = f"new {cls}()"
        else:
            ph = ", ".join(f"${i}" for i in range(n))
            path = f"new {cls}({ph})"
    elif has_this:
        if n == 0:
            path = f"$0.{jn}()"
        else:
            ph = ", ".join(f"${i + 1}" for i in range(n))
            path = f"$0.{jn}({ph})"
    else:
        if js_class or static_of:
            cls = js_class or static_of
            path = f"{cls}.{jn}"
        elif namespace:
            path = f"{namespace}.{jn}"
        else:
            path = jn
    return f'#[js::import = "{path}"]'


def transform_function(macro, other, name, params_str, ret_part):
    import_attr = build_import(macro, params_str)
    js_class = stripq(macro.get("js_class"))
    static_of = stripq(macro.get("static_method_of"))
    _namespace = stripq(macro.get("js_namespace"))
    js_class_for_import = js_class or static_of

    has_this = (
        js_class is not None
        and first_param_name(params_str) == "this"
        and "static_method_of" not in macro
        and "constructor" not in macro
    )
    # method key indicates class method even if no?
    if has_this:
        new_params = "&self"
        rest = remove_first_param(params_str)
        if rest:
            new_params += ", " + rest
    else:
        new_params = params_str.strip()
    sig = f"pub fn {name}({new_params})"
    ret_part = ret_part.strip()
    if ret_part:
        sig += " " + ret_part
    decl = import_attr + "\n" + sig + ' { panic!("web-sys intrinsic") }'
    if first_param_name(params_str) == "this":
        mtp = re.search(r"this\s*:\s*&\s*(\w+)", params_str)
        target = mtp.group(1) if mtp else None
    else:
        target = TYPE_MAP.get(js_class_for_import, js_class_for_import)
    if (js_class or static_of or "constructor" in macro) and target:
        decl = f"impl {target} {{\n    {decl}\n}}"
    if other:
        decl = "\n".join(other) + "\n" + decl
    return decl


def transform_all(text):
    lines = text.splitlines()
    skip = set()
    for i, l in enumerate(lines):
        if l.strip() == "use wasm_bindgen::prelude::*;":
            skip.add(i)
        if l.strip() == "#[wasm_bindgen]":
            skip.add(i)
    i = 0
    while i < len(lines):
        if lines[i].strip() == 'extern "C" {':
            depth = 1
            j = i + 1
            while j < len(lines) and depth > 0:
                depth += lines[j].count("{") - lines[j].count("}")
                j += 1
            skip.add(i)
            if j - 1 >= 0:
                skip.add(j - 1)
            i = j
        else:
            i += 1
    lines = [lines[k] for k in range(len(lines)) if k not in skip]

    out = []
    pending = []
    i = 0
    nlines = len(lines)
    while i < nlines:
        st = lines[i].strip()
        if st.startswith("pub type "):
            decl_lines = [lines[i]]
            while not decl_lines[-1].strip().endswith(";"):
                i += 1
                decl_lines.append(lines[i])
            decl_text = " ".join(x.strip() for x in decl_lines)
            m = re.match(r"pub type\s+(\w+)", decl_text)
            if m:
                macro, other = split_attrs(pending)
                nm = m.group(1)
                out.append(transform_type(macro, other, nm))
                pending = []
                i += 1
                continue
        mname = re.match(r"pub fn\s+(\w+)", st)
        if (
            mname
            and "(" in st
            and not st.endswith(";")
            or (mname and st.endswith(";") and "(" in st)
        ):
            # accumulate multi-line function declaration
            decl_lines = [lines[i]]
            while not decl_lines[-1].strip().endswith(";"):
                i += 1
                decl_lines.append(lines[i])
            decl_text = " ".join(x.strip() for x in decl_lines)
            mname = re.match(r"pub fn\s+(\w+)", decl_text)
            if mname and "(" in decl_text:
                name = mname.group(1)
                ss = decl_text
                idx = ss.find("(")
                depth = 0
                j = idx
                while j < len(ss):
                    if ss[j] == "(":
                        depth += 1
                    elif ss[j] == ")":
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                params = ss[idx + 1 : j]
                rest = ss[j + 1 :].strip()
                if rest.endswith(";"):
                    rest = rest[:-1].strip()
                ret = rest
                macro, other = split_attrs(pending)
                decl = transform_function(macro, other, name, params, ret)
                out.append(decl)
                pending = []
                i += 1
                continue
        pending.append(lines[i])
        i += 1
    out.extend(pending)
    text = "\n".join(out) + "\n"
    # add JsValue import after use super::* if present
    if "use super::*;" in text:
        text = text.replace(
            "use super::*;\n", "use super::*;\nuse ::wasm_bindgen::JsValue;\n", 1
        )
    return text

import argparse
import shutil


def main_cli():
    ap = argparse.ArgumentParser()
    ap.add_argument("--src", default="/tmp/opencode/wbg/web-sys-0.3.80")
    ap.add_argument("--dest", default="support/compat/web-sys")
    args = ap.parse_args()
    src = pathlib.Path(args.src)
    dest = pathlib.Path(args.dest)
    if dest.exists():
        shutil.rmtree(dest)
    shutil.copytree(src, dest)
    # patch Cargo deps
    cargo = dest / "Cargo.toml"
    text = cargo.read_text()
    text = text.replace(
        '[dependencies.js-sys]\nversion = "=0.3.80"\ndefault-features = false',
        '[dependencies.js-sys]\npath = "../js-sys"\nversion = "=0.3.80"\ndefault-features = false',
    )
    text = text.replace(
        '[dependencies.wasm-bindgen]\nversion = "=0.2.103"\ndefault-features = false',
        '[dependencies.wasm-bindgen]\npath = "../wasm-bindgen"\nversion = "=0.2.103"\ndefault-features = false',
    )
    cargo.write_text(text)
    # transform lib + features
    transform_all(dest / "src" / "lib.rs".read_text()) if False else None
    # Use transform_all manually
    (dest / "src" / "lib.rs").write_text(
        transform_all((dest / "src" / "lib.rs").read_text())
    )
    for f in (dest / "src" / "features").glob("*.rs"):
        f.write_text(transform_all(f.read_text()))
    # post: register tool
    lp = dest / "src" / "lib.rs"
    txt = lp.read_text()
    if "register_tool(js)" not in txt:
        txt = txt.replace(
            "#![no_std]\n",
            "#![no_std]\n#![feature(register_tool)]\n#![register_tool(js)]\n",
        )
    # fix window global import

    i = txt.find('#[cfg(feature = "Window")]')
    if i != -1:
        txt = (
            txt[:i]
            + '#[cfg(feature = "Window")]\n#[js::import = "(() => window)"]\npub fn window() -> Option<Window> {\n    panic!("web-sys intrinsic")\n}\n'
        )
    lp.write_text(txt)
    print("refreshed", dest)


if __name__ == "__main__":
    # If invoked as module, keep transform functions but run CLI
    main_cli()
