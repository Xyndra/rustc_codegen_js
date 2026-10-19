use fw_macros::{server, server_main};

#[server]
mod s {
    use framework::{ApiHandler, FnPageRenderer, Method, PageShell, Request, Response};
    use fw_macros::{page, router};
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    const TEMPLATE: &str = include_str!("../index.html");

    struct CounterApi {
        counters: Arc<Mutex<BTreeMap<String, u32>>>,
    }

    impl ApiHandler for CounterApi {
        fn handle_api(&self, req: &Request) -> Option<Response> {
            if req.path.starts_with("/api/counter/") && req.method == Some(Method::Post) {
                let page = req.path.trim_start_matches("/api/counter/");
                let mut c = self.counters.lock().unwrap();
                let n = {
                    let v = c.entry(page.to_string()).or_insert(0);
                    *v += 1;
                    *v
                };
                drop(c);
                return Some(Response::json(200, format!(r#"{{"count":{}}}"#, n)));
            }
            None
        }
    }

    pub fn run() {
        let shell = PageShell::from_bundles(TEMPLATE);
        let counters = Arc::new(Mutex::new(BTreeMap::new()));
        let c = counters.clone();
        let renderer = FnPageRenderer(move |path: &str| {
            let (bundle, name) = match path {
                "/about" => ("about", "about"),
                "/contact" => ("contact", "contact"),
                _ => ("home", "home"),
            };
            let count = c.lock().unwrap().get(name).copied().unwrap_or(0);
            let body = demo_app::render_page_body_count(path, count);
            shell.render(&body, bundle)
        });
        framework::run_site(router!(), renderer, CounterApi { counters }).expect("serve");
    }

    #[page("/")]
    fn home(_req: Request) -> Response {
        Response::ok()
    }

    #[page("/about")]
    fn about(_req: Request) -> Response {
        Response::ok()
    }

    #[page("/contact")]
    fn contact(_req: Request) -> Response {
        Response::ok()
    }
}

#[server_main]
fn main() {
    s::run();
}
