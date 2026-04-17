use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CardBorderStyle {
    Border,
    Dash,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CardSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = CardSize::Md)]
    size: CardSize,
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

    let card_size = match props.size {
        CardSize::Xs => "card-xs",
        CardSize::Sm => "card-sm",
        CardSize::Md => "card-md",
        CardSize::Lg => "card-lg",
        CardSize::Xl => "card-xl",
    };
    rsx! {
        div { class: "card {card_size} {card_border_style} {props.class}", {props.children} }
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

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    rsx! {
        h2 { class: "card-title {props.class}", {props.children} }
    }
}
