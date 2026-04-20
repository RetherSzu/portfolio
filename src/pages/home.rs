use crate::components::icon::{ArrowExternalIcon, ArrowLeftIcon};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "col items-center justify-center h-full",
            div {
                class: "col m-auto mw-640 gap-24",
                div {
                    class: "col m-auto mw-640 gap-24",
                    h1 {
                        class: "space-grotesk text-60 font-bold label primary",
                        style: "letter-spacing: -1.5px",
                        "Hi, I'm Rether. I build game"
                        br {}
                        "engines and high-performance graphics."
                    }
                    p {
                        class: "inter mx-540 label secondary",
                        style: "font-size: 20px; font-weight: 400;",
                        "Focusing on low-level "
                        span {
                            class: "label primary",
                            style: "font-weight: 300;",
                            "C++"
                        }
                        ", "
                        span {
                            class: "label primary",
                            style: "font-weight: 300;",
                            "Vulkan"
                        }
                        ", and custom "
                        span {
                            class: "label primary",
                            style: "font-weight: 400;",
                            "memory allocators"
                        }
                        "."
                    }
                }
                hr {
                    style: "margin: 3rem 0;",
                }
                div {
                    class: "row",
                    div {
                        class: "col flex-1 button bg-secondary label primary border",
                        style: "padding: 24px; gap: 16px; text-align: start",
                        p {
                            class: "jetbrains-mono label secondary",
                            style: "font-size: 10px;",
                            "01 // PROJECTS"
                        }
                        div {
                            class: "row items-center",
                            p {
                                class: "flex-1 inter label primary w-full",
                                style: "font-size: 14px; font-weight: 500",
                                "Engine Projects"
                            }
                            ArrowLeftIcon {
                                class: "icon-arrow"
                            }
                        }
                    }
                    button {
                        class: "col flex-1 button bg-secondary label primary border",
                        style: "padding: 24px; gap: 16px; text-align: start",
                        p {
                            class: "jetbrains-mono label secondary",
                            style: "font-size: 10px;",
                            "02 // ARCHIVE"
                        }
                        div {
                            class: "row items-center",
                            p {
                                class: "flex-1 inter label primary w-full",
                                style: "font-size: 14px; font-weight: 500",
                                "Technical Writing"
                            }
                            ArrowLeftIcon {
                                class: "icon-arrow"
                            }
                        }
                    }
                    button {
                        class: "col flex-1 button bg-secondary label primary border",
                        style: "padding: 24px; gap: 16px; text-align: start",
                        p {
                            class: "jetbrains-mono label secondary",
                            style: "font-size: 10px;",
                            "03 // SOURCE"
                        }
                        div {
                            class: "row items-center",
                            p {
                                class: "inter flex-1 label primary w-full",
                                style: "font-size: 14px; font-weight: 500;",
                                "Github Repo"
                            }
                            ArrowExternalIcon {
                                class: "icon-arrow github"
                            }
                        }
                    }
                }
            }
        }
    }
}