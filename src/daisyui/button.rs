use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonColor {
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = None)]
    color: Option<ButtonColor>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_color = match props.color {
        Some(color) => match color {
            ButtonColor::Neutral => "btn-neutral",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Secondary => "btn-secondary",
            ButtonColor::Accent => "btn-accent",
            ButtonColor::Info => "btn-info",
            ButtonColor::Success => "btn-success",
            ButtonColor::Warning => "btn-warning",
            ButtonColor::Error => "btn-error",
        },
        None => "",
    };
    rsx! {
        button { class: "btn {button_color} {props.class}", {props.children} }
    }
}
