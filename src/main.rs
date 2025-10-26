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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            id: "control_bar",
            class: "bg-blue flex justify-between items-center h-6",
            div {
                class: "flex-1 text-center cursor-move",
                onmousedown: move |_| {
                    dioxus::desktop::window().drag();
                },
                "LOF Launcher"
            },
            button {
                id: "close",
                class: "w-6 h-6 flex items-center justify-center bg-red-400 text-white rounded-full hover:bg-red-500 active:bg-red-600 transition-colors shadow-md text-xs font-bold leading-none",
                onclick: move |_| {
                    window.close();
                },
                "X"
            },
        }
        // 第一行按键 1-5
        div {
            class: "",
            for c in (1..=5) {
                KeyButton { label: c.to_string() }
            }
        }
        // 第二行按键 qwert
        div {
            class: "",
            for c in ['q', 'w', 'e', 'r', 't'] {
                KeyButton { label: c.to_string() }
            }
        }
        // 第三行按键 asdfg
        div {
            class: "",
            for c in ['a', 's', 'd', 'f', 'g'] {
                KeyButton { label: c.to_string() }
            }
        }
        // 第四行按键 zxcvb
        div {
            class: "",
            for c in ['z', 'x', 'c', 'v', 'b'] {
                KeyButton { label: c.to_string() }
            }
        }
    }
}

#[component]
fn KeyButton(label: String) -> Element {
    rsx! {
        button {
            class: "",
            "{label}"
        }
    }
}
