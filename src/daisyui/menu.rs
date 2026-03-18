use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum MenuType {
    Vertical,
    Horizontal,
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    #[props(default = MenuType::Vertical)]
    menu_type: MenuType,
    #[props(default = "".into())]
    class: String,
    children: Element,
}

#[component]
pub fn Menu(props: MenuProps) -> Element {
    let menu_type = match props.menu_type {
        MenuType::Vertical => "menu-vertical",
        MenuType::Horizontal => "menu-horizontal",
    };
    rsx! {
        ul { class: "menu {menu_type} {props.class}", {props.children} }
    }
}
