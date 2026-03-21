use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn List(props: ListProps) -> Element {
    rsx! {
        ul { class: "list {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ListRowProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn ListRow(props: ListRowProps) -> Element {
    rsx! {
        li { class: "list-row {props.class}", {props.children} }
    }
}
