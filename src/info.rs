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
pub struct UserResumeInfo {
    pub skill_section_info: UserResumeSkillSectionInfo,
}

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
                    "NXP Semiconductors (LPC4078)",
                    "ESP8266/12E/32 Wi-Fi Chip",
                    "Raspberry Pi",
                    "Beaglebone Black",
                    "BG96",
                ],
            },
        ],
    };

    let resume_info = UserResumeInfo { skill_section_info };
    UserInfo { resume_info }
}
