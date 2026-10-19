use fw_core::{Component, ComponentId, DirtyMask, RenderCx, RenderScheduler};
use fw_macros::html;
use js_util::closure_to_js_function;
use web_sys::{Event, EventTarget};

pub struct Counter {
    id: ComponentId,
    count: u32,
    dirty: DirtyMask,
    page: &'static str,
}

impl Counter {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            count: 0,
            dirty: DirtyMask::SelfOnly,
            page: "home",
        }
    }
}

impl Component for Counter {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn set_dirty(&mut self, m: DirtyMask) {
        self.dirty = m;
    }
    fn is_dirty(&self) -> DirtyMask {
        self.dirty
    }
    fn render(&mut self, _cx: &mut RenderCx) -> String {
        self.dirty = DirtyMask::None;
        html! {
            <div id="counter" class="p-4 border rounded" data-page={self.page}>
                <button id="inc" class="px-2 py-1 bg-blue-500 text-white rounded">increment</button>
                <span id="count" class="ml-2 font-mono">{self.count}</span>
                <button id="lazy" class="ml-2 px-2 py-1 border rounded">lazy style</button>
            </div>
        }
    }
    fn possible_css_classes(&self) -> &'static str {
        "bg-red-500 text-yellow-300"
    }
}

pub fn html(count: u32, page: &'static str) -> String {
    let mut s = RenderScheduler::new();
    let id = s.allocate_id();
    let mut c = Counter::new(id);
    c.count = count;
    c.page = page;
    let mut cx = RenderCx { scheduler: &mut s };
    c.render(&mut cx)
}

pub fn init() {
    let Some(win) = web_sys::window() else {
        return;
    };
    let Some(doc) = win.document() else {
        return;
    };
    if let Some(btn) = doc.get_element_by_id("inc") {
        let cb = closure_to_js_function(|_e: Event| increment());
        let _ = EventTarget(btn.0).add_event_listener_with_callback("click", &cb);
    }
    if let Some(btn) = doc.get_element_by_id("lazy") {
        let cb = closure_to_js_function(|_e: Event| toggle_lazy());
        let _ = EventTarget(btn.0).add_event_listener_with_callback("click", &cb);
    }
}

fn increment() {
    let Some(win) = web_sys::window() else {
        return;
    };
    let Some(doc) = win.document() else {
        return;
    };
    let page = match doc.query_selector("[data-page]") {
        Ok(Some(el)) => match el.get_attribute("data-page") {
            Some(page) => page,
            None => String::from("home"),
        },
        _ => String::from("home"),
    };
    let url = format!("/api/counter/{page}");
    let promise: js_util::JsValue = js_util::js1("fetch($0, {method:`POST`})", url);
    let cb = closure_to_js_function(move |resp: js_util::JsValue| {
        let json: js_util::JsValue = js_util::js1("$0.json()", resp);
        let cb = closure_to_js_function(move |data: js_util::JsValue| {
            let count: String = js_util::js1("String($0.count)", data);
            if let Some(el) = doc.get_element_by_id("count") {
                el.set_inner_html(&count);
            }
        });
        let _: js_util::JsValue = js_util::js2("$0.then($1)", json, cb);
    });
    let _: js_util::JsValue = js_util::js2("$0.then($1)", promise, cb);
}

fn toggle_lazy() {
    let Some(win) = web_sys::window() else {
        return;
    };
    let Some(doc) = win.document() else {
        return;
    };
    let Some(el) = doc.get_element_by_id("lazy") else {
        return;
    };
    if el.class_name().contains("bg-red-500") {
        el.set_class_name("ml-2 px-2 py-1 border rounded");
    } else {
        el.set_class_name("ml-2 px-2 py-1 border rounded bg-red-500 text-yellow-300");
    }
}
