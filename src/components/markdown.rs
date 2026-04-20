use crate::components::admonition::{admonition_end_html, admonition_start_html};
use dioxus::document::eval;
use dioxus::prelude::*;
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    content: String,
}

#[component]
pub fn MarkdownView(props: MarkdownProps) -> Element {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&props.content, options);

    let mut in_normal_blockquote = false;

    let modified_parser = parser.map(|event| {
        match event {
            Event::Start(Tag::BlockQuote(Some(kind))) => {
                Event::Html(admonition_start_html(&kind).into())
            }

            Event::End(TagEnd::BlockQuote(Some(_))) => {
                Event::Html(admonition_end_html().into())
            }

            Event::Start(Tag::BlockQuote(None)) => {
                in_normal_blockquote = true;
                Event::Start(Tag::BlockQuote(None))
            }

            Event::End(TagEnd::BlockQuote(None)) => {
                in_normal_blockquote = false;
                Event::End(TagEnd::BlockQuote(None))
            }

            _ => event,
        }
    });

    let mut html_output = String::new();

    html::push_html(&mut html_output, modified_parser);

    use_effect(move || {
        let js_code = r#"
            let container = document.getElementById('markdown-content');
            if (!container) return;

            let scripts = container.querySelectorAll('script');

            scripts.forEach(oldScript => {
                let newScript = document.createElement('script');

                Array.from(oldScript.attributes).forEach(attr => {
                    newScript.setAttribute(attr.name, attr.value);
                });

                newScript.text = oldScript.text;

                oldScript.parentNode.replaceChild(newScript, oldScript);
            });
        "#;

        eval(js_code);
    });

    use_effect(move || {
        let _ = props.content.clone();

        let _ = eval(r#"
            setTimeout(() => {
                if (window.hljs) {
                    document.querySelectorAll('#markdown-content pre code').forEach((block) => {
                        hljs.highlightElement(block);
                    });
                } else {
                    console.warn("Highlight.js n'est pas détecté sur la page.");
                }
            }, 10);
        "#);
    });

    rsx! {
        div {
            id: "markdown-content",
            class: "mw-640 m-auto geist-mono col gap-24 label primary p-b-24",
            dangerous_inner_html: "{html_output}"
        }
    }
}