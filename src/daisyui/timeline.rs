use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TimelineType {
    Horizontal,
    Vertical,
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    #[props(default = "".into())]
    class: String,
    #[props(default = false)]
    is_snap_icon: bool,
    #[props(default = false)]
    is_box: bool,
    #[props(default = false)]
    is_compact: bool,
    #[props(default = TimelineType::Horizontal)]
    timeline_type: TimelineType,
    children: Element,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let timeline_type = match props.timeline_type {
        TimelineType::Horizontal => "timeline-horizontal",
        TimelineType::Vertical => "timeline-vertical",
    };
    let timeline_compact = if props.is_compact {
        "timeline-compact"
    } else {
        ""
    };
    let timeline_snap_icon = if props.is_snap_icon {
        "timeline-snap-icon"
    } else {
        ""
    };

    rsx! {
        ul { class: "timeline {timeline_type} {timeline_snap_icon} {timeline_compact} {props.class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineElementProps {
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn TimelineMiddle(props: TimelineElementProps) -> Element {
    rsx! {
        div { class: "timeline-middle {props.class}", {props.children} }
    }
}

#[component]
pub fn TimelineStart(props: TimelineElementProps) -> Element {
    rsx! {
        div { class: "timeline-start {props.class}", {props.children} }
    }
}

#[component]
pub fn TimelineEnd(props: TimelineElementProps) -> Element {
    rsx! {
        div { class: "timeline-end {props.class}", {props.children} }
    }
}
