#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::desktop::{use_window, Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    // 设置窗口标题
                    .with_title("LOF Launcher")
                    // 设置窗口是否在最顶层
                    .with_always_on_top(false)
                    // 设置窗口是否可调整大小
                    .with_resizable(false)
                    // 设置窗口是否有边框装饰
                    .with_decorations(false)
                    // 设置窗口启动是否获得焦点
                    .with_focused(false)
                    .with_inner_size(LogicalSize::new(480, 360)),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let window = use_window();
    rsx! {
        div {
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            div {
                class: "flex justify-center min-h-screen bg-gray-100 p-0",
                div {
                    class: "grid grid-rows-5 gap-2 w-full px-2",
                    // 控制按钮区域 - 位于最上方
                    div {
                        class: "flex justify-between items-center w-full h-8 pt-0",
                        // 拖动按钮 - 位于左上角
                        button {
                            class: "w-6 h-6 flex items-center justify-center bg-gray-400 text-white rounded-full hover:bg-gray-500 active:bg-gray-600 transition-colors shadow-md text-xs font-bold leading-none",
                            onmousedown: move |_| dioxus::desktop::window().drag(),
                            "≡"
                        }
                        // 关闭按钮 - 位于右上角
                        button {
                            class: "w-6 h-6 flex items-center justify-center bg-red-500 text-white rounded-full hover:bg-red-600 active:bg-red-700 transition-colors shadow-md text-sm font-bold leading-none",
                            onclick: move |_| {
                                window.close();
                            },
                            "×"
                        }
                    }
                    // 第一行按键 1-5
                    div {
                        class: "flex gap-2 w-full",
                        for c in (1..=5) {
                            KeyButton { label: c.to_string() }
                        }
                    }
                    // 第二行按键 qwert
                    div {
                        class: "flex gap-2 w-full",
                        for c in ['q', 'w', 'e', 'r', 't'] {
                            KeyButton { label: c.to_string() }
                        }
                    }
                    // 第三行按键 asdfg
                    div {
                        class: "flex gap-2 w-full",
                        for c in ['a', 's', 'd', 'f', 'g'] {
                            KeyButton { label: c.to_string() }
                        }
                    }
                    // 第四行按键 zxcvb
                    div {
                        class: "flex gap-2 w-full",
                        for c in ['z', 'x', 'c', 'v', 'b'] {
                            KeyButton { label: c.to_string() }
                        }
                    }
                }
            }
        }
    }
}


#[component]
fn KeyButton(label: String) -> Element {
    rsx! {
        button {
            class: "px-3 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 active:bg-blue-700 transition-colors shadow-md flex-1 h-12 flex items-center justify-center text-lg font-bold",
            "{label}"
        }
    }
}
