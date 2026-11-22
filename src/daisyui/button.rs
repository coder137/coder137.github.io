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

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonStyle {
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonModifier {
    Wide,
    Block,
    Square,
    Circle,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = None)]
    color: Option<ButtonColor>,
    #[props(default = None)]
    style: Option<ButtonStyle>,
    #[props(default = None)]
    modifier: Option<ButtonModifier>,
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
    let button_style = match props.style {
        Some(style) => match style {
            ButtonStyle::Outline => "btn-outline",
            ButtonStyle::Dash => "btn-dash",
            ButtonStyle::Soft => "btn-soft",
            ButtonStyle::Ghost => "btn-ghost",
            ButtonStyle::Link => "btn-link",
        },
        None => "",
    };
    let button_modifier = match props.modifier {
        Some(modifier) => match modifier {
            ButtonModifier::Wide => "btn-wide",
            ButtonModifier::Block => "btn-block",
            ButtonModifier::Square => "btn-square",
            ButtonModifier::Circle => "btn-circle",
        },
        None => "",
    };
    rsx! {
        button { class: "btn {button_color} {button_style} {button_modifier} {props.class}",
            {props.children}
        }
    }
}
