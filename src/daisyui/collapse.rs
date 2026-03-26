use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CollapseCheckboxIconModifier {
    Arrow,
    Plus,
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapseProps {
    #[props(default = "".into())]
    class: String,
    icon_modifier: Option<CollapseCheckboxIconModifier>,
    children: Element,
}

#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let icon = match props.icon_modifier {
        Some(icon) => match icon {
            CollapseCheckboxIconModifier::Arrow => "collapse-arrow",
            CollapseCheckboxIconModifier::Plus => "collapse-plus",
        },
        None => "",
    };
    rsx! {
        div { class: "collapse {icon} {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapseCheckboxProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = "".into())]
    aria_label: String,
}

#[component]
pub fn CollapseCheckbox(props: CollapseCheckboxProps) -> Element {
    rsx! {
        input {
            r#type: "checkbox",
            aria_label: if !props.aria_label.is_empty() { "{props.aria_label}" },
            class: "{props.class}",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapseTitleProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn CollapseTitle(props: CollapseTitleProps) -> Element {
    rsx! {
        div { class: "collapse-title {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapseContentProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn CollapseContent(props: CollapseContentProps) -> Element {
    rsx! {
        div { class: "collapse-content {props.class}", {props.children} }
    }
}
