use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Use the unified UI App component
        hearth_ui::App {}
    }
}
