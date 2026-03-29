use crate::*;

#[component]
pub fn Header() -> Element {
    let home_route = website::Route::Home.to_string();
    let projects_root_route = website::Route::ProjectsRoot.to_string();
    rsx! {
        header {
            daisyui::Navbar { class: "bg-base-200 rounded-field",
                daisyui::NavbarStart {
                    a {
                        class: "font-barrio tracking-wide text-4xl",
                        href: home_route,
                        "Niket Naidu"
                    }
                }
                daisyui::NavbarEnd {
                    website::Socials {}
                    div { class: "pl-4.5" }
                    daisyui::Divider {
                        class: "py-2 mx-0",
                        divider_type: daisyui::DividerType::Horizontal,
                    }
                    daisyui::Menu { menu_type: daisyui::MenuType::Horizontal,
                        li {
                            a { class: "text-base", href: projects_root_route, "Projects" }
                        }
                    }
                }
            }
        }
    }
}
