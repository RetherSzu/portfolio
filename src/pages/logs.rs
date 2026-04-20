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
pub fn Logs() -> Element {
    let mut sorted_logs: Vec<&LogD> = LOGS.iter().collect();
    sorted_logs.sort_by(|a, b| b.date.cmp(a.date));

    let mut logs_by_year: Vec<(&str, Vec<(&str, &LogD)>)> = Vec::new();

    for log in sorted_logs {
        let year = if log.date.len() >= 4 { &log.date[0..4] } else { log.date };
        let month_day = log.date;

        if let Some(last_group) = logs_by_year.last_mut() {
            if last_group.0 == year {
                last_group.1.push((month_day, log));
                continue;
            }
        }
        logs_by_year.push((year, vec![(month_day, log)]));
    }

    rsx! {
        div {
            class: "mw-640 m-auto",
            style: "padding-bottom: 96px;",
            header {
                class: "col gap-16",
                style: "margin-bottom: 5rem; margin-top: 96px;",
                h1 {
                    class: "geist label primary",
                    style: "font-size: 80px; line-height: 80px; letter-spacing: -4px;",
                    "LOGS"
                }
                div { class: "row items-center gap-16",
                    hr { style: "width: 96px" }
                    span {
                        class: "geist label secondary",
                        style: "font-size: 11px; line-height: 16.5px; letter-spacing: 3.3px;",
                        "Journal & Documentation Archive"
                    }
                }
            }
            section {
                div { class: "container-small", style: "margin-bottom: 48px",
                    div { class: "col", style: "gap: 96px",
                        for (year , year_logs) in logs_by_year {
                            div { class: "col col-100 year-group small-margin-bottom",
                                div {
                                    class: "row space-between items-baseline border-bottom",
                                    style: "padding-bottom: 16px; margin-bottom: 32px",
                                    h2 {
                                        class: "geist-mono label secondary",
                                        style: "font-size: 14px; line-height: 20px; letter-spacing: 1.4px;",
                                        "{year}_CY"
                                    }
                                    p {
                                        class: "geist-mono label tertiary",
                                        style: "font-size: 10px; line-height: 20px; letter-spacing: 1.4px;",
                                        "TOTAL_ENTRIES: {year_logs.len():02}"
                                    }
                                }

                                ul { class: "col gap-4 padding-0",
                                    for (month_day , log) in year_logs {
                                        Link {
                                            class: "row button space-between items-baseline",
                                            style: "padding: 16px 8px;",
                                            to: Route::Log {
                                                id: log.id.to_string(),
                                            },

                                            p {
                                                class: "jetbrains-mono label primary",
                                                style: "font-size: 14px;",
                                                "{log.title}"
                                            }

                                            span {
                                                class: "jetbrains-mono label tertiary",
                                                style: "font-size: 11px; line-height: 16.5px; letter-spacing: 0px;",
                                                "{month_day.replace(\"-\", \".\")}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}