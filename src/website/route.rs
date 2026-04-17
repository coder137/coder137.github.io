use std::sync::LazyLock;

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
    static RESUME_INFO: LazyLock<info::UserResumeInfo> = LazyLock::new(|| info::resume());

    #[derive(Clone, Copy)]
    pub struct UserResumeNavigationInfo {
        section_name: &'static str,
        name: &'static str,
        data: fn(&'static info::UserResumeInfo) -> Element,
    }

    static RESUME_SKILLS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "skills",
        name: "Skill",
        data: |resume| {
            rsx! {
                website::ResumeSkillSection { skills: &resume.skills }
            }
        },
    };

    static RESUME_EXPERIENCE_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "experience",
        name: "Experience",
        data: |resume| {
            rsx! {
                website::ResumeExperienceSection { experience: &resume.experience }
            }
        },
    };

    static RESUME_EDUCATION_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
        section_name: "education",
        name: "Education",
        data: |resume| {
            rsx! {
                website::ResumeEducationSection { education: &resume.education }
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
