#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeAboutInfo {
    pub name: String,
    pub title: String,
    pub purpose: Option<String>,
    pub email: String,
}

#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeSkillInfo {
    pub title: String,
    pub topics: Vec<String>,
}

#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeRoleTitleInfo {
    pub title: String,
    pub start: (u32, u32),
    pub end: Option<(u32, u32)>,
    pub achievements: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub enum ResumeRoleInfo {
    One {
        company: String,
        title: ResumeRoleTitleInfo,
    },
    Many {
        company: String,
        titles: Vec<ResumeRoleTitleInfo>,
    },
}

#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeDegreeInfo {
    pub start: (u32, u32),
    pub end: Option<(u32, u32)>,
    pub university: String,
    pub degree_type: String,
    pub course: String,
    pub specialization: Option<String>,
}

#[derive(Debug, PartialEq, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeProjectInfo {
    pub start: (u32, u32),
    pub end: Option<(u32, u32)>,
    pub title: String,
    pub link: Option<String>,
    pub about: String,
    pub achievements: Vec<String>,
    pub skills: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ResumeInfo {
    pub about: ResumeAboutInfo,
    pub skills: Vec<ResumeSkillInfo>,
    pub roles: Vec<ResumeRoleInfo>,
    pub degrees: Vec<ResumeDegreeInfo>,
    pub projects: Vec<ResumeProjectInfo>,
}
