#[derive(Clone, Copy, PartialEq)]
pub struct UserResumeSkillSectionOneSkillInfo {
    pub title: &'static str,
    pub topics: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserResumeSkillSectionInfo {
    pub skills: &'static [UserResumeSkillSectionOneSkillInfo],
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
    pub skill_section_info: UserResumeSkillSectionInfo,
    pub education: UserEducationInfo,
}

#[derive(Clone, Copy, PartialEq)]
pub struct UserInfo {
    pub resume_info: UserResumeInfo,
}

pub fn info() -> UserInfo {
    let skill_section_info = UserResumeSkillSectionInfo {
        skills: &[
            UserResumeSkillSectionOneSkillInfo {
                title: "Programming Languages",
                topics: &["Rust", "C++17", "C", "Python", "Dart"],
            },
            UserResumeSkillSectionOneSkillInfo {
                title: "OS & Tools",
                topics: &[
                    "FreeRTOS",
                    "Zephyr RTOS",
                    "Linux",
                    "OpenOCD",
                    "Git",
                    "GDB",
                    "CMake",
                    "Flutter",
                    "Dioxus",
                ],
            },
            UserResumeSkillSectionOneSkillInfo {
                title: "Network & Wireless Protocols",
                topics: &[
                    "gRPC",
                    "MQTT",
                    "TCP",
                    "UDP",
                    "QUIC",
                    "BLE",
                    "OpenThread",
                    "GPS",
                ],
            },
            UserResumeSkillSectionOneSkillInfo {
                title: "Hardware Interfaces",
                topics: &["I2C", "SPI", "CAN", "UART"],
            },
            UserResumeSkillSectionOneSkillInfo {
                title: "Processors",
                topics: &[
                    "STMicroelectronics (STM32F7/STM32L4)",
                    "Texas Instruments (TM4C123G/CC1352)",
                    "BG96",
                    "Raspberry Pi",
                    "Beaglebone Black",
                    "NXP Semiconductors (LPC4078)",
                    "ESP8266/12E/32 Wi-Fi Chip",
                ],
            },
        ],
    };
    let education = UserEducationInfo {
        degrees: &[
            UserOneEducationInfo {
                start: 2019,
                end: Some(2021),
                university: "San Jose State University",
                degree_type: "Master of Science",
                course: "Computer Engineering",
                specialization: Some("Embedded Systems"),
            },
            UserOneEducationInfo {
                start: 2013,
                end: Some(2017),
                university: "Amity University",
                degree_type: "Bachelor of Technology",
                course: "Electronics and Communication",
                specialization: None,
            },
        ],
    };

    let resume_info = UserResumeInfo {
        skill_section_info,
        education,
    };
    UserInfo { resume_info }
}
