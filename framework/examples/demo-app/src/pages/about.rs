use crate::common::counter;
use fw_macros::html;

pub fn render(count: u32) -> String {
    html! {
        <main class="max-w-2xl mx-auto p-6" data-page="about">
            <h1 class="text-2xl font-bold mb-4">about</h1>
            <p class="mb-4 text-gray-600">page: about</p>
            {counter::html(count, "about")}
        </main>
    }
}

pub fn init() {
    crate::common::counter::init();
}
