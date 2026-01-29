use dioxus::prelude::*;

use crate::components::auth::Auth;

pub mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_SCSS: Asset = asset!("/assets/styles/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }

        Auth {

        }
    }
}
