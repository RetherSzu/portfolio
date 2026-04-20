use crate::components::icon::{get_svg_html_string, CAUTION_PATHS, IMPORTANT_PATHS, NOTE_PATHS, TIP_PATHS, WARNING_PATHS};
use pulldown_cmark::BlockQuoteKind;

pub fn admonition_start_html(kind: &BlockQuoteKind) -> String {
    let (icon_svg, title, css_class) = match kind {
        BlockQuoteKind::Note => (get_svg_html_string(NOTE_PATHS, "0 0 24 24"), "Note", "note"),
        BlockQuoteKind::Tip => (get_svg_html_string(TIP_PATHS, "0 0 24 24"), "Tip", "tip"),
        BlockQuoteKind::Important => (get_svg_html_string(IMPORTANT_PATHS, "0 0 24 24"), "Important", "important"),
        BlockQuoteKind::Warning => (get_svg_html_string(WARNING_PATHS, "0 0 24 24"), "Warning", "warning"),
        BlockQuoteKind::Caution => (get_svg_html_string(CAUTION_PATHS, "0 0 24 24"), "Caution", "caution"),
    };

    format!(
        r#"
        <blockquote class="admonition {}">
            <div class="admonition-header">
                <span class="icon-wrapper">{}</span>
                <strong>{} :</strong>
            </div>
            <div class="admonition-content">
        "#,
        css_class, icon_svg, title
    )
}

pub fn admonition_end_html() -> &'static str {
    "</div></blockquote>\n"
}