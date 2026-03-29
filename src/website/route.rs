use std::sync::LazyLock;

pub use crate::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/projects")]
    ProjectsRoot,

    #[route("/projects/:project")]
    ProjectSpecific { project: String },
}

async fn create_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    let mut routes = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    // Dynamic routes
    // Get some random project ids
    let ids: [i32; 3] = [1, 123, 1357];
    let mut dynamic_routes = ids
        .into_iter()
        .map(|id| {
            Route::ProjectSpecific {
                project: format!("{id}"),
            }
            .to_string()
        })
        .collect::<Vec<_>>();

    routes.append(&mut dynamic_routes);
    Ok(routes)
}

#[cfg(feature = "server")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    create_routes().await
}

#[component]
fn Home() -> Element {
    static RESUME_INFO: LazyLock<info::UserResumeInfo> = LazyLock::new(|| info::resume());

    #[derive(Clone, Copy)]
    pub struct UserResumeNavigationInfo {
        section_name: &'static str,
        name: &'static str,
        data: fn(&'static info::UserResumeInfo) -> Element,
    }

    static RESUME_SKILLS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "resume-skills",
        name: "Skill",
        data: |resume| {
            rsx! {
                navigation::ResumeSkillSection { skills: &resume.skills }
            }
        },
    };

    static RESUME_EXPERIENCE_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "resume-experience",
        name: "Experience",
        data: |resume| {
            rsx! {
                navigation::ResumeExperienceSection { experience: &resume.experience }
            }
        },
    };

    static RESUME_EDUCATION_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "resume-education",
        name: "Education",
        data: |resume| {
            rsx! {
                navigation::ResumeEducationSection { education: &resume.education }
            }
        },
    };

    static RESUME_PROJECTS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "resume-projects",
        name: "Projects",
        data: |resume| {
            rsx! {
                navigation::ResumeProjectsSection { projects: &resume.projects }
            }
        },
    };

    static RESUME_NAVIGATION: &[UserResumeNavigationInfo] = &[
        RESUME_SKILLS_NAVIGATION,
        RESUME_EXPERIENCE_NAVIGATION,
        RESUME_EDUCATION_NAVIGATION,
        RESUME_PROJECTS_NAVIGATION,
    ];

    let resume = &RESUME_INFO;
    let resume_navigation = RESUME_NAVIGATION;
    let navigation = rsx! {
        for rn in resume_navigation {
            li {
                a { href: "#{rn.section_name}", class: "text-lg", "{rn.name}" }
            }
        }
    };

    let data = rsx! {
        for rn in resume_navigation {
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

#[component]
fn ProjectsRoot() -> Element {
    rsx! {
        div { class: "grid md:grid-cols-2 gap-4 mb-4 mt-4",
            daisyui::Card {
                class: "bg-base-200",
                size: daisyui::CardSize::Lg,
                border: daisyui::CardBorderStyle::Border,
                daisyui::CardBody { class: "items-center",
                    daisyui::CardTitle { "Title" }
                    div { class: "flex flex-wrap gap-2 justify-center" }
                }
            }

            daisyui::Card {
                class: "bg-base-200 image-full",
                size: daisyui::CardSize::Lg,
                border: daisyui::CardBorderStyle::Border,
                // figure {
                //     class: "",
                //     img {
                //         class: "aspect-square scale-75 object-scale-down px-0 py-0",
                //         src: asset!("/assets/projects/enterprise-software-dev.png")
                //     }
                // }
                daisyui::CardBody { class: "items-center justify-center text-center",
                    daisyui::CardTitle { "Enterprise Firmware platform development" }
                    p {
                        "To create an enterprise-level firmware stack from scratch using the GCC ARM toolchain"
                    }
                    div { class: "flex flex-wrap gap-2 justify-center" }
                }
                        // div {
            //     class: "card-actions justify-end",
            //     button {
            //         class: "btn btn-primary",

            //     }
            // }
            }
        }
    }
}

#[component]
fn ProjectSpecific(project: String) -> Element {
    rsx! { "User page for user with project: {project}" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_routes() {
        let routes = create_routes().await.unwrap();
        println!("Data: {routes:?}");
    }
}
