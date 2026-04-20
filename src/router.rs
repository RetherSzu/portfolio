use crate::components::layout::Layout;
use dioxus::prelude::*;

use crate::pages::{
    home::Home,
    log::Log,
    logs::Logs,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]

    #[route("/")]
    Home {},

    #[route("/logs")]
    Logs {},

    #[route("/log/:id")]
    Log {
        id: String
    },

    #[route("/:..segments")]
    PageNotFound {
        segments: Vec<String>
    },
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
                class: "col items-center justify-center h-full gap-24",
                h1 { class: "geist label primary", "404 - PAGE NOT FOUND" }
                Link {
                    to: Route::Home {},
                    class: "geist button p-16",
                    "Back to home"
                }
            }
    }
}