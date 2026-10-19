use fw_macros::client;

#[client]
pub mod common;
#[client]
pub mod pages;

#[client]
pub fn render_page_body(path: &str) -> String {
    render_page_body_count(path, 0)
}

#[client]
pub fn render_page_body_count(path: &str, count: u32) -> String {
    pages::render(path, count)
}

#[client]
pub fn init() {
    pages::init();
}
