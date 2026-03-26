pub use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = false)]
    allow_html_in_markdown: bool,
    content: String,
}

#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let mut gfm_markdown_options = markdown::Options::gfm();
    gfm_markdown_options.compile.allow_dangerous_html = props.allow_html_in_markdown;
    gfm_markdown_options.compile.gfm_tagfilter = !props.allow_html_in_markdown;

    let html_buf = markdown::to_html_with_options(&props.content, &gfm_markdown_options).unwrap();
    rsx! {
        div { class: "{props.class}", dangerous_inner_html: "{html_buf}" }
    }
}
