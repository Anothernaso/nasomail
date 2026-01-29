use dioxus::prelude::*;

#[component]
pub fn Auth() -> Element {
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
