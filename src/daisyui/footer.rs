use dioxus::prelude::*;

// pub enum FooterDirection {
//     Vertical,
//     Horizontal,
// }

#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = false)]
    center: bool,
    children: Element,
}

#[component]
pub fn Footer(props: FooterProps) -> Element {
    let center = if props.center { "footer-center" } else { "" };
    rsx! {
        footer { class: "footer {center} {props.class}", {props.children} }
    }
}
