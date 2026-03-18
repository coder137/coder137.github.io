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

        div { class: "mx-auto max-w-5xl p-2",
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
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Experience" }
        }
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Education" }
        }
    }
}

#[component]
fn ResumeSkillSection(skill_section_info: info::UserResumeSkillSectionInfo) -> Element {
    let is_odd = skill_section_info.skills.len() % 2 != 0;
    let last = skill_section_info.skills.len() - 1;
    rsx! {
        div { class: "grid md:grid-cols-4 gap-4 mb-4",
            if is_odd {
                for (i , s) in skill_section_info.skills.into_iter().enumerate() {
                    if i == last {
                        ResumeOneSkill {
                            class: "md:col-start-2 col-span-2",
                            skill_info: *s,
                        }
                    } else {
                        ResumeOneSkill { class: "col-span-2", skill_info: *s }
                    }
                }
            } else {
                for s in skill_section_info.skills {
                    ResumeOneSkill { class: "col-span-2", skill_info: *s }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneSkillProps {
    #[props(default = "".into())]
    class: String,
    skill_info: info::UserResumeSkillSectionOneSkillInfo,
}

#[component]
fn ResumeOneSkill(props: ResumeOneSkillProps) -> Element {
    rsx! {
        daisyui::Card {
            class: "bg-base-200 {props.class}",
            border: daisyui::CardBorderStyle::Border,
            daisyui::CardBody { class: "items-center",
                daisyui::CardTitle { text: "{props.skill_info.title}" }
                div { class: "flex flex-wrap gap-2 justify-center",
                    for s in props.skill_info.topics {
                        daisyui::Badge { text: s, color: daisyui::BadgeColor::Accent }
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
                    WebsiteSocials { }
                }
            }
        }
    }
}

#[component]
pub fn WebsiteFooter() -> Element {
    rsx! {
        daisyui::Footer {
            center: false,
            class: "footer-horizontal flex justify-between items-center p-2",
            p { class: "text-base", "\u{00A9} 2025 Niket Naidu. All rights reserved." }
            WebsiteSocials {}
        }
    }
}

#[component]
pub fn WebsiteSocials() -> Element {
    rsx! {
        daisyui::Menu { class: "gap-2", menu_type: daisyui::MenuType::Horizontal,
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
