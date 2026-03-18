use dioxus::prelude::*;

mod daisyui;

mod info;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Debug, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
}

#[cfg(feature = "server")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

fn main() {
    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }

    // https://dioxuslabs.com/learn/0.7/essentials/fullstack/static_site_generation/
    #[cfg(feature = "server")]
    {
        dioxus::LaunchBuilder::new()
            // Set the server config only if we are building the server target
            .with_cfg(server_only! {
                ServeConfig::builder()
                    // Enable incremental rendering
                    .incremental(
                        dioxus::server::IncrementalRendererConfig::new()
                            // Store static files in the public directory where other static assets like wasm are stored
                            .static_dir(
                                std::env::current_exe()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .join("public")
                            )
                            // Don't clear the public folder on every build. The public folder has other files including the wasm
                            // binary and static assets required for the app to run
                            .clear_cache(false)
                    )
                    .enable_out_of_order_streaming()
            })
            .launch(App);
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Meta { name: "description", content: "coder137.portfolio" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "mx-auto max-w-5xl",
            WebsiteHeader {}
            Router::<Route> {}
            WebsiteFooter {}
        }
    }
}

#[component]
pub fn Home() -> Element {
    let info = info::info();
    rsx! {
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Skills" }
        }
        ResumeSkillSection { skill_section_info: info.resume_info.skill_section_info }
    }
}

#[component]
fn ResumeSkillSection(skill_section_info: info::UserResumeSkillSectionInfo) -> Element {
    let is_odd = skill_section_info.skills.len() % 2 != 0;
    let last = skill_section_info.skills.len() - 1;
    rsx! {
        div { class: "grid md:grid-cols-4 md:gap-4 mb-2",
            if is_odd {
                for (i , rs) in skill_section_info.skills.into_iter().enumerate() {
                    if i == last {
                        ResumeOneSkill {
                            class: "col-start-2 col-span-2",
                            title: rs.title,
                            skills: rs.topics,
                        }
                    } else {
                        ResumeOneSkill {
                            class: "col-span-2",
                            title: rs.title,
                            skills: rs.topics,
                        }
                    }
                }
            } else {
                for rs in skill_section_info.skills {
                    ResumeOneSkill {
                        class: "col-span-2",
                        title: rs.title,
                        skills: rs.topics,
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneSkillProps {
    #[props(default = "".into())]
    class: String,
    title: &'static str,
    skills: &'static [&'static str],
}

#[component]
fn ResumeOneSkill(props: ResumeOneSkillProps) -> Element {
    rsx! {
        daisyui::Card { class: props.class, border: daisyui::CardBorderStyle::Border,
            daisyui::CardBody { class: "items-center",
                div { class: "card-title", "{props.title}" }
                div { class: "flex flex-wrap gap-2 justify-center",
                    for s in props.skills {
                        daisyui::Badge { text: s, color: daisyui::BadgeColor::Primary }
                    }
                }
            }
        }
    }
}

#[component]
pub fn WebsiteHeader() -> Element {
    rsx! {
        header {
            daisyui::Navbar { class: "",
                daisyui::NavbarStart {
                    a { class: "text-3xl font-black", href: "/", "Niket Naidu" }
                }
                daisyui::NavbarEnd {
                    daisyui::Menu {
                        class: "gap-2",
                        menu_type: daisyui::MenuType::Horizontal,
                        a { href: "https://www.linkedin.com/in/niket-naidu/",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaLinkedin,
                            }
                        }

                        a { href: "https://github.com/coder137",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub,
                            }
                        }

                        a { href: "mailto:niketnaiduus@gmail.com",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaGoogle,
                            }
                        }
                    }

                }
            }
        }
    }
}

#[component]
pub fn WebsiteFooter() -> Element {
    rsx! {
        daisyui::Footer { center: true, class: "footer-horizontal bg-base-200 p-2",
            aside { class: "grid-flow-col items-center",
                p { class: "text-base", "\u{00A9} 2025 Niket Naidu. All right reserved." }
            }

            div { class: "grid grid-flow-col gap-4",
                a { href: "https://www.linkedin.com/in/niket-naidu/",
                    dioxus_free_icons::Icon {
                        width: 24,
                        height: 24,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaLinkedin,
                    }
                }

                a { href: "https://github.com/coder137",
                    dioxus_free_icons::Icon {
                        width: 24,
                        height: 24,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub,
                    }
                }

                a { href: "mailto:niketnaiduus@gmail.com",
                    dioxus_free_icons::Icon {
                        width: 24,
                        height: 24,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaGoogle,
                    }
                }
            }
        }
    }
}
