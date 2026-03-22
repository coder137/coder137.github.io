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

#[derive(Clone, Copy)]
pub struct UserResumeNavigationInfo {
    section_name: &'static str,
    name: &'static str,
    data: fn(&'static UserResumeInfo) -> Element,
}

static RESUME_SKILLS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
    section_name: "resume-skills",
    name: "Skill",
    data: |resume| {
        rsx! {
            ResumeSkillSection { skills: &resume.skills }
        }
    },
};

static RESUME_EXPERIENCE_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
    section_name: "resume-experience",
    name: "Experience",
    data: |resume| {
        rsx! {
            ResumeExperienceSection { experience: &resume.experience }
        }
    },
};

static RESUME_EDUCATION_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
    section_name: "resume-education",
    name: "Education",
    data: |resume| {
        rsx! {
            ResumeEducationSection { education: &resume.education }
        }
    },
};

static RESUME_PROJECTS_NAVIGATION: UserResumeNavigationInfo = UserResumeNavigationInfo {
    section_name: "resume-projects",
    name: "Projects",
    data: |resume| {
        rsx! {
            ResumeProjectsSection { projects: &resume.projects }
        }
    },
};

static RESUME_NAVIGATION: &[UserResumeNavigationInfo] = &[
    RESUME_SKILLS_NAVIGATION,
    RESUME_EXPERIENCE_NAVIGATION,
    RESUME_EDUCATION_NAVIGATION,
    RESUME_PROJECTS_NAVIGATION,
];

#[component]
fn Home() -> Element {
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
            {(rn.data)(&resume)}
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

#[derive(Props, Clone, PartialEq)]
struct ResumeOneSkillProps {
    #[props(default = "".into())]
    class: String,
    skill: &'static info::UserOneSkillInfo,
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
fn ResumeSkillSection(skills: &'static info::UserSkillInfo) -> Element {
    let is_odd = skills.skills.len() % 2 != 0;
    let last = skills.skills.len() - 1;
    rsx! {
        div { class: "grid md:grid-cols-4 gap-4 mb-4",
            if is_odd {
                for (i , skill) in skills.skills.iter().enumerate() {
                    if i == last {
                        ResumeOneSkill { class: "md:col-start-2 col-span-2", skill }
                    } else {
                        ResumeOneSkill { class: "col-span-2", skill }
                    }
                }
            } else {
                for skill in skills.skills {
                    ResumeOneSkill { class: "col-span-2", skill }
                }
            }
        }
    }
}

pub fn to_month_str(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

#[component]
pub fn YearAndMonth(start: (u32, u32), end: Option<(u32, u32)>) -> Element {
    let (start_year, start_month) = start;
    let start_month_str = to_month_str(start_month);
    let start = rsx! { "{start_month_str} {start_year}" };

    let end = match end {
        Some((end_year, end_month)) => {
            let end_month_str = to_month_str(end_month);
            rsx! { "{end_month_str} {end_year}" }
        }
        None => rsx! { "Present" },
    };

    rsx! {
        div { class: "text-base-content/50",
            time { {start} }
            " - "
            {end}
        }
    }
}

#[component]
fn ResumeOneExperienceTitle(
    left: bool,
    title: &'static info::UserOneExperienceTitleInfo,
) -> Element {
    let text_direction = if left { "md:text-right" } else { "" };
    let row_direction = if left { "md:flex-row-reverse" } else { "" };
    rsx! {
        p { class: "font-bold", "{title.title}" }
        YearAndMonth { start: title.start, end: title.end }
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
    experience: &'static info::UserOneExperienceInfo,
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
            for (i , title) in titles.iter().enumerate() {
                ResumeOneExperienceTitle { left: props.left, title }
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
            for (i , role) in experience.roles.iter().enumerate() {
                ResumeOneExperience {
                    left: i % 2 == 0,
                    start: i != 0,
                    end: i != (experience.roles.len() - 1),
                    experience: role,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResumeOneProjectProps {
    left: bool,
    start: bool,
    end: bool,
    project: &'static info::UserOneProjectInfo,
}

#[component]
fn ResumeOneProject(props: ResumeOneProjectProps) -> Element {
    let title_and_link = match props.project.link {
        Some(link) => {
            rsx! {
                a { class: "link", href: "{link}", "{props.project.title}" }
            }
        }
        None => {
            rsx! {
                a { "{props.project.title}" }
            }
        }
    };

    let row_direction = if props.left {
        "md:flex-row-reverse"
    } else {
        ""
    };

    let info = rsx! {
        div { class: "text-lg font-bold pb-2", {title_and_link} }

        div { class: "flex {row_direction} flex-wrap gap-2",
            daisyui::Badge {
                text: "{props.project.project_type_tag}",
                color: daisyui::BadgeColor::Primary,
            }
            daisyui::Badge {
                text: "{props.project.project_type:?}",
                color: daisyui::BadgeColor::Primary,
            }
        }

        YearAndMonth { start: props.project.start, end: props.project.end }

        {props.project.about}
    };
    rsx! {
        li {
            if props.start {
                hr { class: "bg-primary" }
            }

            daisyui::TimelineMiddle {
                dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::fa_solid_icons::FaMicrochip }
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
fn ResumeProjectsSection(projects: &'static info::UserProjectInfo) -> Element {
    rsx! {
        daisyui::Timeline {
            class: "max-md:timeline-compact",
            timeline_type: daisyui::TimelineType::Vertical,
            is_snap_icon: true,
            is_compact: false,
            for (i , project) in projects.projects.iter().enumerate() {
                ResumeOneProject {
                    left: i % 2 == 0,
                    start: i != 0,
                    end: i != (projects.projects.len() - 1),
                    project,
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
    degree: &'static info::UserOneEducationInfo,
}

#[component]
fn ResumeOneEducation(props: ResumeOneEducationProps) -> Element {
    let end = match props.degree.end {
        Some(end) => rsx! {
            time { "{end}" }
        },
        None => rsx! { "Present" },
    };
    let specialization = match props.degree.specialization {
        Some(specialization) => rsx! {
            br {}
            p { class: "italic", "{specialization}" }
        },
        None => rsx! {},
    };
    let info = rsx! {
        div { class: "text-base-content/50",
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
            for (i , degree) in education.degrees.iter().enumerate() {
                ResumeOneEducation {
                    left: i % 2 == 0,
                    start: i != 0,
                    end: i != (education.degrees.len() - 1),
                    degree,
                }
            }
        }
    }
}
