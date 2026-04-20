use crate::components::markdown::MarkdownView;
use crate::router::Route;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LogD {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub date: &'static str,
    pub content: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/generated_logs.rs"));

#[component]
pub fn Log(id: String) -> Element {
    let Some(log) = LOGS.iter().find(|p| p.id == id) else {
        return rsx! {
            div {
                class: "col items-center justify-center h-full gap-24",
                h1 { class: "geist label primary", "404 - LOG NOT FOUND" }
                p {
                    class: "geist-mono label secondary",
                    "The entry "
                    code {"{id}"}
                    " does not exist or has been deleted."
                }
                Link {
                    to: Route::Logs {},
                    class: "geist button p-16",
                    "Back to logs"
                }
            }
        };
    };

    rsx! {
        header {
            class: "border-bottom p-b-24 m-b-48 mw-640",
            style: "margin: 96px auto",
		    div {
                class: "row gap-24",
                div {
                    class: "row",
                    p {
                        class: "geist-mono label primary",
                        "LOG_{log.id:0>2}"
                    }
                }
                div {
                    class: "row gap-4",
                    p {
                        class: "geist-mono label secondary",
                        "UPDT:"
                    }
                    p {
                        class: "geist-mono label primary",
                        "{log.date}"
                    }
                }
		    }
        }

        section {
            class: "border-bottom m-b-48 mw-640 m-auto gap-24 col justify-center",
            h1 {
                class: "geist label primary",
                style: "font-size: 48px; word-break: break-all;",
                "{log.title}"
            }
            hr {
                class: "title"
            }
        }

        MarkdownView {
            content: log.content.to_string()
        }
    }
}