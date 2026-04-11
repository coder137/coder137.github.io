use dioxus::prelude::*;

use crate::{daisyui, info};

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
            size: daisyui::CardSize::Lg,
            border: daisyui::CardBorderStyle::Border,
            daisyui::CardBody { class: "items-center",
                daisyui::CardTitle { "{props.skill.title}" }
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
pub fn ResumeSkillSection(skills: &'static info::UserSkillInfo) -> Element {
    let is_odd = skills.skills.len() % 2 != 0;
    let last = skills.skills.len() - 1;
    rsx! {
        div { class: "grid md:grid-cols-4 gap-4 mb-4",
            if is_odd {
                for (i, skill) in skills.skills.iter().enumerate() {
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

fn to_month_str(month: u32) -> &'static str {
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
fn YearAndMonth(start: (u32, u32), end: Option<(u32, u32)>) -> Element {
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

    let collapse_arrow_start = if left {
        "md:after:right-1 md:pr-6 max-md:after:left-0 max-md:after:end-auto max-md:pl-5"
    } else {
        "after:left-0 after:end-auto pl-5"
    };
    rsx! {
        p { class: "", "{title.title}" }
        YearAndMonth { start: title.start, end: title.end }
        daisyui::Collapse { icon_modifier: daisyui::CollapseCheckboxIconModifier::Plus,
            daisyui::CollapseCheckbox { aria_label: "Achievements" }
            daisyui::CollapseTitle { class: "{collapse_arrow_start} font-semibold pb-2 pt-0 after:top-0",
                "Achievements"
            }
            daisyui::CollapseContent {
                daisyui::List { class: "{text_direction} text-base",
                    for (i, achievement) in title.achievements.iter().enumerate() {
                        if i == 0 {
                            daisyui::ListRow { class: "pt-0 px-0 gap-0",
                                crate::ui::Markdown {
                                    class: "prose dark:prose-invert text-base-content",
                                    content: "{achievement}",
                                }
                            }
                        } else {
                            daisyui::ListRow { class: "px-0 gap-0",
                                crate::ui::Markdown {
                                    class: "prose dark:prose-invert text-base-content",
                                    content: "{achievement}",
                                }
                            }
                        }
                    }
                }
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
fn WebsiteTimelineTitle(title: String) -> Element {
    rsx! {
        div { class: "text-xl font-semibold", "{title}" }
    }
}

#[component]
fn ResumeOneExperience(props: ResumeOneExperienceProps) -> Element {
    let info = match props.experience {
        info::UserOneExperienceInfo::One { company, title } => {
            rsx! {
                WebsiteTimelineTitle { title: company }
                ResumeOneExperienceTitle { left: props.left, title }
            }
        }
        info::UserOneExperienceInfo::Many { company, titles } => rsx! {
            WebsiteTimelineTitle { title: company }
            for (i, title) in titles.iter().enumerate() {
                ResumeOneExperienceTitle { left: props.left, title }
                if i != titles.len() - 1 {
                    daisyui::Divider {}
                }
            }
        },
    };
    let timeline_options = "pb-4 mb-4 timeline-box bg-base-200 text-base";
    rsx! {
        li {
            if props.start {
                hr { class: "bg-primary" }
            }

            daisyui::TimelineMiddle {
                dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::fa_solid_icons::FaBriefcase }
            }

            if props.left {
                daisyui::TimelineStart { class: "md:text-end {timeline_options}", {info} }
            } else {
                daisyui::TimelineEnd { class: "{timeline_options}", {info} }
            }

            if props.end {
                hr { class: "bg-primary" }
            }
        }
    }
}

#[component]
pub fn ResumeExperienceSection(experience: &'static info::UserExperienceInfo) -> Element {
    rsx! {
        daisyui::Timeline {
            class: "max-md:timeline-compact",
            timeline_type: daisyui::TimelineType::Vertical,
            is_snap_icon: true,
            is_compact: false,
            for (i, role) in experience.roles.iter().enumerate() {
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
        WebsiteTimelineTitle { title: "{props.degree.university}" }
        p { class: "font-bold", "{props.degree.degree_type}" }
        "{props.degree.course}"
        {specialization}
    };
    let timeline_options = "pb-4 mb-4 timeline-box bg-base-200 text-base";
    rsx! {
        li {
            if props.start {
                hr { class: "bg-primary" }
            }

            daisyui::TimelineMiddle {
                dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::fa_solid_icons::FaGraduationCap }
            }

            if props.left {
                daisyui::TimelineStart { class: "md:text-end {timeline_options}", {info} }
            } else {
                daisyui::TimelineEnd { class: "{timeline_options}", {info} }
            }

            if props.end {
                hr { class: "bg-primary" }
            }
        }
    }
}

#[component]
pub fn ResumeEducationSection(education: &'static info::UserEducationInfo) -> Element {
    rsx! {
        daisyui::Timeline {
            class: "max-md:timeline-compact",
            timeline_type: daisyui::TimelineType::Vertical,
            is_snap_icon: true,
            is_compact: false,
            for (i, degree) in education.degrees.iter().enumerate() {
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

#[component]
pub fn ResumeOneProject(project: &'static info::UserOneProjectInfo) -> Element {
    rsx! {
        daisyui::Card {
            class: "bg-base-200",
            size: daisyui::CardSize::Lg,
            border: daisyui::CardBorderStyle::Border,
            daisyui::CardBody { class: "items-center justify-center text-center",
                daisyui::CardTitle {
                    "{project.title}"
                    if let Some(link) = project.link {
                        a { href: "{link}", aria_label: "Project link",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_solid_icons::FaLink,
                            }
                        }
                    }
                }
                YearAndMonth { start: project.start, end: project.end }
                div { class: "flex flex-wrap gap-2 justify-center",
                    for tag in project.tags {
                        daisyui::Badge { text: tag, color: daisyui::BadgeColor::Primary }
                    }
                }
                p { {project.about} }
            }
        }
    }
}

#[component]
pub fn ResumeProjectsSection(projects: &'static info::UserProjectInfo) -> Element {
    rsx! {
        div { class: "grid md:grid-cols-1 gap-4 mb-4",
            for project in projects.projects {
                ResumeOneProject { project }
            }
        }
    }
}
