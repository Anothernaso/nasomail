use dioxus::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

pub type AuthContextPtr = Arc<RwLock<AuthContext>>;

#[derive(Clone)]
pub struct AuthContext {
    pub state: AuthState,
}

#[derive(Clone)]
pub enum AuthState {
    Home,
    Login,
    Register,
}

#[component]
pub fn Auth() -> Element {
    let ctx = use_context_provider(|| {
        Arc::new(RwLock::new(AuthContext {
            state: AuthState::Home,
        }))
    });

    let login_ctx = ctx.clone();
    let login = move |_| {
        let mut ctx = login_ctx.write().unwrap();
        ctx.state = AuthState::Login;
    };

    let register_ctx = ctx.clone();
    let register = move |_| {
        let mut ctx = register_ctx.write().unwrap();
        ctx.state = AuthState::Register;
    };

    rsx! {
        if matches!(ctx.read().unwrap().state, AuthState::Home) {
            h2 { "Authenticate" }
            button {
                onclick: login,
                "Log In"
            }
            br {}
            button {
                onclick: register,
                "Register"
            }
        }

        if matches!(ctx.read().unwrap().state, AuthState::Login) {
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
