use dioxus::prelude::*;

use crate::{components::auth::Auth, state::AppState};

pub mod components;
pub mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_SCSS: Asset = asset!("/assets/styles/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let state = use_context_provider(|| AppState::Auth);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }

        if matches!(state, AppState::Auth) {
            Auth {}
        }
    }
}
