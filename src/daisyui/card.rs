use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CardBorderStyle {
    Border,
    Dash,
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default = "".into())]
    class: String,
    border: Option<CardBorderStyle>,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let card_border_style = match props.border {
        Some(border_style) => match border_style {
            CardBorderStyle::Border => "card-border",
            CardBorderStyle::Dash => "card-dash",
        },
        None => "",
    };
    rsx! {
        div { class: "card {card_border_style} {props.class}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        div { class: "card-body {props.class}", {props.children} }
    }
}
