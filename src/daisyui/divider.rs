use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    rsx! {
        div { class: "divider {props.class}", {props.children} }
    }
}
