use crate::components::navbar::Navbar;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "row h-full",

            Navbar {}

            main {
                style: "margin-left: 290px; width: 100%; padding: 1rem;",
                Outlet::<Route> {}
            }
        }
    }
}