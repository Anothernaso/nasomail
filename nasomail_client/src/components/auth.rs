use dioxus::prelude::*;

#[derive(Clone)]
pub enum AuthState {
    Home,
    Login,
    Register,
}

#[component]
pub fn Auth() -> Element {
    let state = use_context_provider(|| AuthState::Home);

    rsx! {
        if matches!(state, AuthState::Home) {
            h2 { "Authenticate" }
            button { "Log In" }
            br {}
            button { "Register" }
        }

        if matches!(state, AuthState::Login) {
            Login {}
        }
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        h2 { "Log In" }

        input {
            id: "username_input",
            type: "text",
            placeholder: "Username",
        }

        br {  }

        input {
            id: "password_input",
            type: "password",
            placeholder: "Password",
        }
    }
}
