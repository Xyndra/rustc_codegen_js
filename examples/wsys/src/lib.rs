#![feature(register_tool)]
#![register_tool(js)]

use js_util::{js0, js1, js2, js3, JsValue};
use web_sys::window as window_fn;
use web_sys::EventTarget;
use web_sys::{Document, Element};

pub fn write_status(d: &Document, msg: &str) {
    if let Some(el) = d.get_element_by_id("status") {
        el.set_inner_html(msg);
    }
}

#[js::export]
pub fn init() -> JsValue {
    let w = window_fn().unwrap();
    let d = w.document().unwrap();
    write_status(&d, "booting Rust / web-sys / Three.js...");
    if let Some(stage) = d.get_element_by_id("stage") {
        stage.set_inner_html("");
        let canvas = d.create_element("canvas").unwrap();
        let _ = canvas.set_attribute("width", "640");
        let _ = canvas.set_attribute("height", "360");
        let _ = canvas.set_attribute("id", "gl");
        let _ = stage.insert_adjacent_element("beforeend", &canvas);

        let renderer: JsValue = js1(
            "new THREE.WebGLRenderer({ canvas: $0, antialias: true })",
            &canvas,
        );
        let _: () = js1("$0.setSize(640, 360)", renderer);

        let root: JsValue = js0("new THREE.Scene()");
        let _: () = js1("$0.background = new THREE.Color(0x111827)", root);

        let camera: JsValue = js0("new THREE.PerspectiveCamera(60, 640 / 360, 0.1, 100)");
        let _: () = js1("$0.position.z = 3", camera);

        let light: JsValue = js0("new THREE.DirectionalLight(0xffffff, 1)");
        let _: () = js1("$0.position.set(2, 2, 2)", light);
        let _: () = js2("$0.add($1)", root, light);
        let _: () = js2(
            "$0.add($1)",
            root,
            js0::<JsValue>("new THREE.AmbientLight(0xffffff, 0.35)"),
        );

        let geometry: JsValue = js0("new THREE.BoxGeometry(1, 1, 1)");
        let material: JsValue = js0("new THREE.MeshStandardMaterial({ color: 0x60a5fa })");
        let cube: JsValue = js2("new THREE.Mesh($0, $1)", geometry, material);
        let _: () = js2("$0.add($1)", root, cube);

        let _: () = js3("$0.render($1, $2)", renderer, root, camera);

        let _ = stage.set_attribute("tabindex", "0");
        let _: () = js1("$0.focus()", stage);
        let cb = js_util::closure_to_js_function(move |e: web_sys::KeyboardEvent| {
            let _ = on_key(e.key(), renderer, root, camera, cube);
        });
        let et = EventTarget(w.0);
        let _ = et.add_event_listener_with_callback("keydown", &cb);
        write_status(&d, "three ready");
        JsValue::null()
    } else {
        JsValue::null()
    }
}

#[js::export]
pub fn on_key(
    key: String,
    renderer: JsValue,
    root: JsValue,
    camera: JsValue,
    cube: JsValue,
) -> String {
    let w = window_fn().unwrap();
    let d = w.document().unwrap();
    let k = key.to_lowercase();
    let message = match k.as_str() {
        "arrowleft" | "a" => {
            let _: () = js1("$0.position.x -= 0.12", cube);
            String::from("moved left")
        }
        "arrowright" | "d" => {
            let _: () = js1("$0.position.x += 0.12", cube);
            String::from("moved right")
        }
        "arrowup" | "w" => {
            let _: () = js1("$0.position.y += + 0.12", cube);
            String::from("moved up")
        }
        "arrowdown" | "s" => {
            let _: () = js1("$0.position.y -= 0.12", cube);
            String::from("moved down")
        }
        "+" | "=" => {
            let _: () = js1("$0.scale.multiplyScalar(1.1)", cube);
            String::from("scaled up")
        }
        "-" | "_" => {
            let _: () = js1("$0.scale.multiplyScalar(0.9)", cube);
            String::from("scaled down")
        }
        "r" => {
            let _: () = js1("$0.position.set(0, 0, 0)", cube);
            let _: () = js1("$0.scale.set(1, 1, 1)", cube);
            String::from("reset")
        }
        other => format!("key: {}", other),
    };
    let _: () = js3("$0.render($1, $2)", renderer, root, camera);
    write_status(&d, &message);
    message
}

#[js::export]
pub fn run() {
    init();
}

#[js::export]
pub fn touch_element(el: &Element) -> String {
    format!("tag={}", el.tag_name())
}
