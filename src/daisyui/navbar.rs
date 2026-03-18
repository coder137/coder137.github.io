use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        div { class: "navbar {props.class}", {props.children} }
    }
}

#[component]
pub fn NavbarStart(children: Element) -> Element {
    rsx! {
        div { class: "navbar-start", {children} }
    }
}

#[component]
pub fn NavbarCenter(children: Element) -> Element {
    rsx! {
        div { class: "navbar-center", {children} }
    }
}

#[component]
pub fn NavbarEnd(children: Element) -> Element {
    rsx! {
        div { class: "navbar-end", {children} }
    }
}
