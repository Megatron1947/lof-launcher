#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::desktop::tao::dpi::PhysicalPosition;
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
                    .with_position(PhysicalPosition::new(1018, 600))
                    .with_inner_size(LogicalSize::new(525, 240)),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            id: "control_bar",
            class: "bg-blue flex justify-between items-center h-6 mb-2",
            div {
                class: "flex-1 text-center cursor-move",
                onmousedown: move |_| {
                    dioxus::desktop::window().drag();
                },
                "LOF Launcher"
            }
            button {
                id: "mini",
                class: "i-mynaui-minus w-6 h-6 flex items-center justify-center bg-red-400 hover:bg-red-500 active:bg-red-600 transition-colors mr-1",
                onclick: move |_| {
                    // 最小化
                    dioxus::desktop::window().set_minimized(true);
                    // 隐藏窗口
                    // dioxus::desktop::window().set_visible(false);
                },
            }
            button {
                id: "close",
                class: "i-mynaui-x w-6 h-6 flex items-center justify-center bg-red-400 hover:bg-red-500 active:bg-red-600 transition-colors mr-1",
                onclick: move |_| {
                    dioxus::desktop::window().close();
                },
            }
        }
        // 第一行按键 1-5
        /*
        div { class: "flex mb-2 ml-2 gap-2",
            for c in (1..=5) {
                NumberButton { label: c.to_string() }
            }
        }
        */
        // 第二行按键 qwert
        div { class: "flex mb-2 ml-2 gap-2",
            for c in ['q', 'w', 'e', 'r', 't'] {
                KeyButton { label: c.to_string() }
            }
        }
        // 第三行按键 asdfg
        div { class: "flex mb-2 ml-2 gap-2",
            for c in ['a', 's', 'd', 'f', 'g'] {
                KeyButton { label: c.to_string() }
            }
        }
        // 第四行按键 zxcvb
        div { class: "flex mb-2 ml-2 gap-2",
            for c in ['z', 'x', 'c', 'v', 'b'] {
                KeyButton { label: c.to_string() }
            }
        }
    }
}

#[component]
fn NumberButton(label: String) -> Element {
    let num = match label.as_str() {
        "1" => "one",
        "2" => "two",
        "3" => "three",
        "4" => "four",
        "5" => "five",
        _ => "",
    };
    rsx! {
        div { class: "border border-gray-300 rounded p-1 flex",
            button { class: "i-mynaui-{num} h-6 w-6" }
            button { class: "h-16 w-16", id: label, "" }
        }
    }
}

#[component]
fn KeyButton(label: String) -> Element {
    rsx! {
        div { class: "border border-gray-300 rounded p-1 flex",
            button { class: "i-mynaui-letter-{label} h-6 w-6" }
            button { class: "h-16 w-16", id: label, "" }
        }
    }
}
