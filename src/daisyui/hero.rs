use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    rsx! {
        div { class: "hero {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HeroContentProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn HeroContent(props: HeroContentProps) -> Element {
    rsx! {
        div { class: "hero-content {props.class}", {props.children} }
    }
}
