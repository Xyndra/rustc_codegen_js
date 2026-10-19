use crate::common::counter;
use fw_macros::html;

pub fn render(count: u32) -> String {
    html! {
        <main class="max-w-2xl mx-auto p-6" data-page="home">
            <h1 class="text-2xl font-bold mb-4">home</h1>
            <p class="mb-4 text-gray-600">page: home</p>
            {counter::html(count, "home")}
        </main>
    }
}

pub fn init() {
    crate::common::counter::init();
}
