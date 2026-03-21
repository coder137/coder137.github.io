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
pub struct UserOneExperienceInfo {
    pub start: &'static str,
    pub end: Option<&'static str>,
    pub company: &'static str,
    pub title: &'static str,
    pub achievements: &'static [&'static str],
    pub skills: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserExperienceInfo {
    pub roles: &'static [UserOneExperienceInfo],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserOneEducationInfo {
    pub start: usize,
    pub end: Option<usize>,
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
