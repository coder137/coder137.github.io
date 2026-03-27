#[derive(Clone, Copy, PartialEq)]
pub struct UserOneSkillInfo {
    pub title: &'static str,
    pub topics: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserSkillInfo {
    pub skills: &'static [UserOneSkillInfo],
}

#[derive(Clone, Copy, PartialEq)]
pub enum UserOneExperienceInfo {
    One {
        company: &'static str,
        title: UserOneExperienceTitleInfo,
    },
    Many {
        company: &'static str,
        titles: &'static [UserOneExperienceTitleInfo],
    },
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserOneExperienceTitleInfo {
    pub title: &'static str,
    pub start: (u32, u32),
    pub end: Option<(u32, u32)>,
    pub achievements: &'static [&'static str],
    pub skills: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserExperienceInfo {
    pub roles: &'static [UserOneExperienceInfo],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserOneEducationInfo {
    pub start: u32,
    pub end: Option<u32>,
    pub university: &'static str,
    pub degree_type: &'static str,
    pub course: &'static str,
    pub specialization: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserEducationInfo {
    pub degrees: &'static [UserOneEducationInfo],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserOneProjectInfo {
    pub start: (u32, u32),
    pub end: Option<(u32, u32)>,
    pub title: &'static str,
    pub link: Option<&'static str>,
    pub about: &'static str,
    pub achievements: &'static [&'static str],
    pub skills: &'static [&'static str],
    pub tags: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserProjectInfo {
    pub projects: &'static [UserOneProjectInfo],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserResumeInfo {
    pub skills: UserSkillInfo,
    pub experience: UserExperienceInfo,
    pub education: UserEducationInfo,
    pub projects: UserProjectInfo,
}
