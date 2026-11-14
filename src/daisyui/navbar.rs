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

#[derive(Props, Clone, PartialEq)]
pub struct NavbarStartProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn NavbarStart(props: NavbarStartProps) -> Element {
    rsx! {
        div { class: "navbar-start {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarCenterProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn NavbarCenter(props: NavbarCenterProps) -> Element {
    rsx! {
        div { class: "navbar-center {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarEndProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn NavbarEnd(props: NavbarEndProps) -> Element {
    rsx! {
        div { class: "navbar-end {props.class}", {props.children} }
    }
}
