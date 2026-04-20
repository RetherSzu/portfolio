use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    let mut is_dark = use_signal(|| false);

    use_future(move || async move {
        let mut js_eval = eval(r#"
            const savedTheme = localStorage.getItem('theme');
            const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

            let isDarkTheme = false;

            if (savedTheme) {
                isDarkTheme = savedTheme === 'dark';
            } else {
                isDarkTheme = mediaQuery.matches;
            }

            dioxus.send(isDarkTheme);

            if (isDarkTheme) {
                document.body.classList.add('dark-mode');
            } else {
                document.body.classList.add('light-mode');
            }

            mediaQuery.addEventListener('change', (event) => {
                dioxus.send(event.matches);

                if (event.matches) {
                    document.body.classList.remove('light-mode');
                    document.body.classList.add('dark-mode');
                } else {
                    document.body.classList.remove('dark-mode');
                    document.body.classList.add('light-mode');
                }
            });
        "#);

        while let Ok(is_dark_system) = js_eval.recv::<bool>().await {
            is_dark.set(is_dark_system);
        }
    });

    let set_dark = move |_| {
        is_dark.set(true);
        let _ = eval("localStorage.setItem('theme', 'dark'); document.body.classList.remove('light-mode'); document.body.classList.add('dark-mode');");
    };

    let set_light = move |_| {
        is_dark.set(false);
        let _ = eval("localStorage.setItem('theme', 'light'); document.body.classList.remove('dark-mode'); document.body.classList.add('light-mode');");
    };

    rsx! {
        div {
            class: "row gap-8",
            button {
                onclick: set_dark,
                id: "theme-toggle-btn",
                class: if is_dark() { "link link-active jetbrains-mono" } else { "link jetbrains-mono" },
                "[Dark]"
            }
            p {
                class: "label secondary",
                style: "cursor: default;",
                "/"
            }
             button {
                onclick: set_light,
                id: "theme-toggle-btn",
                class: if !is_dark() { "link link-active jetbrains-mono" } else { "link jetbrains-mono" },
                "[Light]"
            }
        }
    }
}