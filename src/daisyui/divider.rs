use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum DividerType {
    Vertical,
    Horizontal,
}

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = DividerType::Vertical)]
    divider_type: DividerType,
    children: Element,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    let divider_type = match props.divider_type {
        DividerType::Vertical => "divider-vertical",
        DividerType::Horizontal => "divider-horizontal",
    };
    rsx! {
        div { class: "divider {divider_type} {props.class}", {props.children} }
    }
}
