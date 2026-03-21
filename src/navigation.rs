use std::sync::LazyLock;

use dioxus::prelude::*;

use crate::{
    daisyui,
    info::{self, UserResumeInfo},
};

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
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

static RESUME_INFO: LazyLock<UserResumeInfo> = LazyLock::new(|| info::resume());

// TODO, Later migrate this under the `Resume` route
#[component]
fn Home() -> Element {
    let resume = &RESUME_INFO;
    rsx! {
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Skills" }
        }
        ResumeSkillSection { skill: &resume.skill }
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Experience" }
        }
        ResumeExperienceSection { experience: &resume.experience }
        daisyui::Divider {
            h1 { class: "text-2xl font-bold", "Education" }
        }
        ResumeEducationSection { education: &resume.education }
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
fn ResumeSkillSection(skill: &'static info::UserSkillInfo) -> Element {
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

#[component]
fn ResumeOneExperienceTitle(left: bool, title: info::UserOneExperienceTitleInfo) -> Element {
    let end = match title.end {
        Some(end) => rsx! {
            time { "{end}" }
        },
        None => rsx! { "Current" },
    };
    let text_direction = if left { "md:text-right" } else { "" };
    let row_direction = if left { "md:flex-row-reverse" } else { "" };
    rsx! {
        p { class: "font-bold", "{title.title}" }
        div { class: "font-mono italic",
            time { "{title.start}" }
            " - "
            {end}
        }
        daisyui::List { class: "{text_direction} text-base",
            for achievement in title.achievements {
                daisyui::ListRow { class: "px-0 gap-0", "{achievement}" }
            }
        }
        div { class: "flex {row_direction} flex-wrap gap-2",
            for s in title.skills {
                daisyui::Badge { text: s, color: daisyui::BadgeColor::Primary }
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
    let info = match props.experience {
        info::UserOneExperienceInfo::Individual { company, title } => {
            rsx! {
                div { class: "text-lg font-bold", "{company}" }
                ResumeOneExperienceTitle { left: props.left, title }
            }
        }
        info::UserOneExperienceInfo::Group { company, titles } => rsx! {
            div { class: "text-lg font-bold", "{company}" }
            for (i , title) in titles.into_iter().enumerate() {
                ResumeOneExperienceTitle { left: props.left, title: *title }
                if i != titles.len() - 1 {
                    daisyui::Divider {}
                }
            }
        },
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
fn ResumeExperienceSection(experience: &'static info::UserExperienceInfo) -> Element {
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
        div { class: "font-mono italic",
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
fn ResumeEducationSection(education: &'static info::UserEducationInfo) -> Element {
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
