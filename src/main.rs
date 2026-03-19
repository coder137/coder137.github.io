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
        ResumeSkillSection { skill: info.resume.skill }
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Experience" }
        }
        ResumeExperienceSection { experience: info.resume.experience }
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Education" }
        }
        ResumeEducationSection { education: info.resume.education }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneSkillProps {
    #[props(default = "".into())]
    class: String,
    skill: info::UserOneSkillInfo,
}

#[component]
fn ResumeOneSkill(props: ResumeOneSkillProps) -> Element {
    rsx! {
        daisyui::Card {
            class: "bg-base-200 {props.class}",
            border: daisyui::CardBorderStyle::Border,
            daisyui::CardBody { class: "items-center",
                daisyui::CardTitle { text: "{props.skill.title}" }
                div { class: "flex flex-wrap gap-2 justify-center",
                    for s in props.skill.topics {
                        daisyui::Badge { text: s, color: daisyui::BadgeColor::Primary }
                    }
                }
            }
        }
    }
}

#[component]
fn ResumeSkillSection(skill: info::UserSkillInfo) -> Element {
    let is_odd = skill.skills.len() % 2 != 0;
    let last = skill.skills.len() - 1;
    rsx! {
        div { class: "grid md:grid-cols-4 gap-4 mb-4",
            if is_odd {
                for (i , skill) in skill.skills.into_iter().enumerate() {
                    if i == last {
                        ResumeOneSkill { class: "md:col-start-2 col-span-2", skill: *skill }
                    } else {
                        ResumeOneSkill { class: "col-span-2", skill: *skill }
                    }
                }
            } else {
                for skill in skill.skills {
                    ResumeOneSkill { class: "col-span-2", skill: *skill }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneExperienceProps {
    left: bool,
    start: bool,
    end: bool,
    experience: info::UserOneExperienceInfo,
}

#[component]
fn ResumeOneExperience(props: ResumeOneExperienceProps) -> Element {
    let end = match props.experience.end {
        Some(end) => rsx! {
            time { "{end}" }
        },
        None => rsx! { "Current" },
    };
    let text_direction = if props.left { "md:text-right" } else { "" };
    let row_direction = if props.left {
        "md:flex-row-reverse"
    } else {
        ""
    };
    let info = rsx! {
        div { class: "font-mono italic",
            time { "{props.experience.start}" }
            " - "
            {end}
        }
        div { class: "text-lg font-bold", "{props.experience.company}" }
        p { class: "font-bold", "{props.experience.title}" }
        ul { class: "list {text_direction} text-base",
            for achievement in props.experience.achievements {
                li { class: "list-row px-0 gap-0", "{achievement}" }
            }
        }
        div { class: "flex {row_direction} flex-wrap gap-2",
            for s in props.experience.skills {
                daisyui::Badge { text: s, color: daisyui::BadgeColor::Primary }
            }
        }
    };
    rsx! {
        li {
            if props.start {
                hr { class: "bg-primary" }
            }

            daisyui::TimelineMiddle {
                dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::fa_solid_icons::FaBriefcase }
            }

            if props.left {
                daisyui::TimelineStart { class: "md:text-end mb-4 timeline-box bg-base-200 text-base",
                    {info}
                }
            } else {
                daisyui::TimelineEnd { class: "mb-4 timeline-box bg-base-200 text-base", {info} }
            }

            if props.end {
                hr { class: "bg-primary" }
            }
        }
    }
}

#[component]
fn ResumeExperienceSection(experience: info::UserExperienceInfo) -> Element {
    rsx! {
        daisyui::Timeline {
            class: "max-md:timeline-compact",
            timeline_type: daisyui::TimelineType::Vertical,
            is_snap_icon: true,
            is_compact: false,
            for (i , role) in experience.roles.into_iter().enumerate() {
                ResumeOneExperience {
                    left: i % 2 == 0,
                    start: i != 0,
                    end: i != (experience.roles.len() - 1),
                    experience: *role,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneEducationProps {
    #[props(default = "".into())]
    class: String,
    left: bool,
    start: bool,
    end: bool,
    degree: info::UserOneEducationInfo,
}

#[component]
fn ResumeOneEducation(props: ResumeOneEducationProps) -> Element {
    let end = match props.degree.end {
        Some(end) => rsx! {
            time { "{end}" }
        },
        None => rsx! { "Current" },
    };
    let specialization = match props.degree.specialization {
        Some(specialization) => rsx! {
            br {}
            p { class: "italic", "{specialization}" }
        },
        None => rsx! {},
    };
    let info = rsx! {
        div { class: "font-mono italic mt-0.5",
            time { "{props.degree.start}" }
            " - "
            {end}
        }
        div { class: "text-lg font-bold", "{props.degree.university}" }
        p { class: "font-bold", "{props.degree.degree_type}" }
        "{props.degree.course}"
        {specialization}
    };
    rsx! {
        li {
            if props.start {
                hr { class: "bg-primary" }
            }

            daisyui::TimelineMiddle {
                dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::fa_solid_icons::FaGraduationCap }
            }

            if props.left {
                daisyui::TimelineStart { class: "md:text-end mb-4", {info} }
            } else {
                daisyui::TimelineEnd { class: "mb-4", {info} }
            }

            if props.end {
                hr { class: "bg-primary" }
            }
        }
    }
}

#[component]
fn ResumeEducationSection(education: info::UserEducationInfo) -> Element {
    rsx! {
        daisyui::Timeline {
            class: "max-md:timeline-compact",
            timeline_type: daisyui::TimelineType::Vertical,
            is_snap_icon: true,
            is_compact: false,
            for (i , degree) in education.degrees.into_iter().enumerate() {
                ResumeOneEducation {
                    left: i % 2 == 0,
                    start: i != 0,
                    end: i != (education.degrees.len() - 1),
                    degree: *degree,
                }
            }
        }
    }
}

#[component]
pub fn WebsiteHeader() -> Element {
    rsx! {
        header {
            daisyui::Navbar { class: "bg-base-200 rounded-field",
                daisyui::NavbarStart {
                    a { class: "text-3xl font-black", href: "/", "Niket Naidu" }
                }
                daisyui::NavbarEnd { WebsiteSocials {} }
            }
        }
    }
}

#[component]
pub fn WebsiteFooter() -> Element {
    rsx! {
        daisyui::Footer {
            class: "footer-horizontal bg-base-200 rounded-field flex justify-between items-center px-2",
            center: false,
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
