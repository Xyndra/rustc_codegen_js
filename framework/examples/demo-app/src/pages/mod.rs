use fw_macros::page_feature;

#[page_feature(home)]
pub mod home;
#[page_feature(about)]
pub mod about;
#[page_feature(contact)]
pub mod contact;
pub mod not_found;

pub fn render(path: &str, count: u32) -> String {
    match path {
        "/" => render_home(count),
        "/about" => render_about(count),
        "/contact" => render_contact(count),
        _ => not_found::render(count),
    }
}

#[cfg(feature = "page_home")]
fn render_home(count: u32) -> String {
    home::render(count)
}
#[cfg(not(feature = "page_home"))]
fn render_home(_count: u32) -> String {
    String::new()
}

#[cfg(feature = "page_about")]
fn render_about(count: u32) -> String {
    about::render(count)
}
#[cfg(not(feature = "page_about"))]
fn render_about(_count: u32) -> String {
    String::new()
}

#[cfg(feature = "page_contact")]
fn render_contact(count: u32) -> String {
    contact::render(count)
}
#[cfg(not(feature = "page_contact"))]
fn render_contact(_count: u32) -> String {
    String::new()
}

pub fn init() {
    #[cfg(feature = "page_home")]
    home::init();
    #[cfg(feature = "page_about")]
    about::init();
    #[cfg(feature = "page_contact")]
    contact::init();
}
