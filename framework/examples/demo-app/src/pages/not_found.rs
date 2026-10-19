use fw_macros::html;

pub fn render(_count: u32) -> String {
    html! {
        <main class="max-w-2xl mx-auto p-6">
            <h1 class="text-2xl font-bold mb-4">404</h1>
            <p class="mb-4 text-gray-600">not found</p>
        </main>
    }
}
