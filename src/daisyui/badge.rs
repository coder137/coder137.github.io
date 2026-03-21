pub use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeColor {
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
pub struct BadgeProps {
    text: String,
    color: Option<BadgeColor>,
    #[props(default = "".into())]
    class: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    // <div class="badge badge-primary">Primary</div>
    let badge_color = match props.color {
        Some(color) => match color {
            BadgeColor::Neutral => "badge-neutral",
            BadgeColor::Primary => "badge-primary",
            BadgeColor::Secondary => "badge-secondary",
            BadgeColor::Accent => "badge-accent",
            BadgeColor::Info => "badge-info",
            BadgeColor::Success => "badge-success",
            BadgeColor::Warning => "badge-warning",
            BadgeColor::Error => "badge-error",
        },
        None => "",
    };
    rsx! {
        div { class: "badge {badge_color} {props.class}", {props.text} }
    }
}
