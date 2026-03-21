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
    Individual {
        company: &'static str,
        title: UserOneExperienceTitleInfo,
    },
    Group {
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
pub struct UserResumeInfo {
    pub skill: UserSkillInfo,
    pub experience: UserExperienceInfo,
    pub education: UserEducationInfo,
}
