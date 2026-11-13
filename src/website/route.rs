use crate::info::{ABOUT_IMAGE, RESUME_INFO};
pub use crate::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,
    // #[route("/projects")]
    // ProjectsRoot,

    // #[route("/projects/:project")]
    // ProjectSpecific { project: String },
}

async fn create_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    let routes = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        // .chain([1, 123, 1357].into_iter().map(|d| d.to_string()))
        .collect::<Vec<_>>();
    Ok(routes)
}

#[cfg(feature = "server")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    create_routes().await
}

#[component]
fn Home() -> Element {
    #[derive(Clone, Copy)]
    pub struct UserResumeNavigationInfo {
        section_name: &'static str,
        name: &'static str,
        data: fn(&'static schema::ResumeInfo) -> Element,
    }

    static RESUME_ABOUT_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "about",
        name: "About",
        data: |resume| {
            let about = &resume.about;
            rsx! {
                div { class: "hero",
                    div { class: "hero-content w-full flex-col lg:flex-row",
                        img {
                            class: "sm:max-w-3xs max-sm:max-w-5/10 rounded-full",
                            src: ABOUT_IMAGE,
                        }
                        div { class: "pr-4" }
                        div { class: "pr-4  text-center",
                            h1 { class: "relative inline-block text-4xl sm:text-6xl md:text-7xl leading-tight text-base-content group",

                                // Base layer: Darumadrop One stays visible most of the time & locks layout size
                                span { class: "font-darumadrop-one motion-safe:animate-glitch-base block whitespace-nowrap group-hover:animate-none group-hover:opacity-0",
                                    "{about.name}"
                                }

                                // Glitch layer: Rubik Glitch snaps in briefly with zero layout shift
                                span {
                                    aria_hidden: "true",
                                    class: "font-rubik-glitch opacity-0 motion-safe:animate-glitch-burst pointer-events-none select-none absolute inset-y-0 -inset-x-8 flex items-center justify-center whitespace-nowrap group-hover:animate-none group-hover:opacity-100",
                                    "{about.name}"
                                }
                            }
                            div { class: "text-xl font-semibold pb-4", "{about.title}" }
                        }
                    }
                }
            }
        },
    };

    static RESUME_SKILLS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "skills",
        name: "Skill",
        data: |resume| {
            rsx! {
                website::ResumeSkillsSection { skills: &resume.skills }
            }
        },
    };

    static RESUME_ROLES_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "experience",
        name: "Experience",
        data: |resume| {
            rsx! {
                website::ResumeRolesSection { roles: &resume.roles }
            }
        },
    };

    static RESUME_EDUCATION_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "education",
        name: "Education",
        data: |resume| {
            rsx! {
                website::ResumeEducationSection { degrees: &resume.degrees }
            }
        },
    };

    static RESUME_PROJECTS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "projects",
        name: "Projects",
        data: |resume| {
            rsx! {
                website::ResumeProjectsSection { projects: &resume.projects }
            }
        },
    };

    static RESUME_NAVIGATION: &[UserResumeNavigationInfo] = &[
        RESUME_ABOUT_NAVIGATION,
        RESUME_SKILLS_NAVIGATION,
        RESUME_ROLES_NAVIGATION,
        RESUME_EDUCATION_NAVIGATION,
        RESUME_PROJECTS_NAVIGATION,
    ];

    let resume = &*RESUME_INFO;
    let navigation = rsx! {
        for rn in RESUME_NAVIGATION {
            li {
                a { href: "#{rn.section_name}", class: "text-lg", "{rn.name}" }
            }
        }
    };

    let data = rsx! {
        for rn in RESUME_NAVIGATION {
            div { id: "{rn.section_name}", class: "scroll-mt-20" }
            daisyui::Divider {
                h1 { class: "text-2xl font-bold", "{rn.name}" }
            }
            {(rn.data)(resume)}
        }
    };

    rsx! {
        daisyui::Navbar { class: "bg-base-100 sticky top-0 z-50",
            daisyui::NavbarStart {}
            daisyui::NavbarCenter {
                daisyui::Menu { menu_type: daisyui::MenuType::Horizontal, {navigation} }
            }
            daisyui::NavbarEnd {}
        }
        {data}
    }
}

// #[component]
// fn ProjectsRoot() -> Element {
//     rsx! {
//         div { class: "grid md:grid-cols-2 gap-4 mb-4 mt-4",
//             daisyui::Card {
//                 class: "bg-base-200",
//                 size: daisyui::CardSize::Lg,
//                 border: daisyui::CardBorderStyle::Border,
//                 figure { class: "p-4",
//                     img {
//                         class: "aspect-video rounded-lg",
//                         src: asset!("/assets/projects/vishnu-mohanan-1-unsplash.jpg"),
//                     }
//                 }
//                 daisyui::CardBody { class: "items-center justify-center text-center pt-0 pb-4",
//                     daisyui::CardTitle { "Enterprise Firmware platform development" }
//                     p {
//                         "To create an enterprise-level firmware stack from scratch using the GCC ARM toolchain"
//                     }
//                     div { class: "pb-2" }
//                     div { class: "card-actions",
//                         button { class: "btn btn-primary", "Read More" }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[component]
// fn ProjectSpecific(project: String) -> Element {
//     rsx! { "User page for user with project: {project}" }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_routes() {
        let routes = create_routes().await.unwrap();
        println!("Data: {routes:?}");
    }
}
