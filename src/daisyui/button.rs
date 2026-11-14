use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button { class: "btn {props.class}", {props.children} }
    }
}
