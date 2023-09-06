#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch(App);
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(App);
}
pub fn App(cx: Scope) -> Element {
    render!(
        style { include_str!("style.css") }
        label { class: "switch",
            input { r#type: "checkbox" }
            span { class: "slider round" }
        }
    )
}
