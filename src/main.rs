use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            div {
                class: "p-4 flex items-center justify-center min-h-screen bg-gray-100",
                button {
                    class: "px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 active:bg-blue-700 transition-colors shadow-md",
                    "测试UnoCSS"
                }
            }
        }
    }
}
