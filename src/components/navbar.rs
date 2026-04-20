use crate::components::icon::{HomeIcon, WritingIcon};
use crate::components::theme_toggle::ThemeToggle;
use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "navbar padded-section p-24 stroke-right bg-primary",
            style: "position: fixed; height: calc(100% - 48px);",

            div {
                class: "col h-full",
                style: "gap: 48px",

                div {
                    class: "stroke-bottom",
                    style: "padding-bottom: 1.5rem",
                    Link {
                        to: Route::Home {},
                        class: "label primary space-grotesk",
                        style: "font-weight: bold; font-size: 18px; line-height: 28px; letter-spacing: -0.9px; margin-bottom: 4px",
                        "ENGINE_DEV"
                    }
                }

                div {
                    class: "col gap-8 flex-1",

                    Link {
                        to: Route::Home {},
                        class: "nav-link row items-center gap-12",
                        active_class: "active",
                        HomeIcon {
                            class: "label secondary"
                        }

                        p {
                            class: "space-grotesk label secondary",
                            "Home"
                        }
                    }

                    Link {
                        to: Route::Logs {},
                        class: "nav-link row items-center gap-12",
                        active_class: "active",
                        WritingIcon {
                            class: "label secondary"
                        }

                        p {
                            class: "space-grotesk label secondary",
                            "Logs"
                        }
                    }
                }

                div {
                    class: "col gap-16 stroke-top",
                    style: "padding-top: 24px",
                    ThemeToggle {}
                }
            }
        }
    }
}