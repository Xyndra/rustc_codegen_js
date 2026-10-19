use crate::common::counter;
use fw_macros::html;

pub fn render(count: u32) -> String {
    html! {
        <main class="max-w-2xl mx-auto p-6" data-page="contact">
            <h1 class="text-2xl font-bold mb-4">contact</h1>
            <p class="mb-4 text-gray-600">page: contact</p>
            {counter::html(count, "contact")}
        </main>
    }
}

pub fn init() {
    crate::common::counter::init();
}
