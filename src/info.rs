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

#[derive(Clone, Copy, PartialEq)]
pub struct UserInfo {
    pub resume: UserResumeInfo,
}

#[rustfmt::skip]
const QUALCOMM_SENIOR_ENGINEER_ACHIEVEMENTS: &[&str] = &[
"Currently working on accelerating AI inference performance on Qualcomm GPUs to enable real-time, on-device execution of complex neural networks.",
"Authored high-performance rust modules for encryption, logging, and created vital Rust bindings for the Qualcomm Cloud AI software stack to enable advanced AI workloads.",
"Pioneered new strategies to enhance codebase concurrency and parallelism, as well as creating novel visualizations with distributed tracing to optimize performance."
];

#[rustfmt::skip]
const QUALCOMM_ENGINEER_ACHIEVEMENTS: &[&str] = &[
"Contributed significantly to a 4G LTE project, integrating a Qualcomm Modem with Oneweb's LEO satellite grid by implementing diverse features and resolving complex bugs across the Middle Layer (ML1) and Firmware (FW) teams.",
"Drove the development of GNSS Cold Start independently, coordinating closely with cross-functional teams to integrate it within the ML1 and FW architectures.",
"Enhanced developer efficiency by proactively redesigning the logging system to automatically visualize asynchronous system interactions, resulting in multiple hours saved in debugging.",
// "Designed a automated ontarget jenkins CI system which"
];

#[rustfmt::skip]
const TURINGSENSE_INTERN_ACHIEVEMENTS: &[&str] = &[
"Architectured performant drivers (UART, SPI, TWI) during board bring-up on the Nordic ecosystem.",
"Designed 3 unique software prototypes to speed up BLE transfer speeds from 10kbps to 1.4mbps.",
];

#[rustfmt::skip]
const BLUERIVER_INTERN_ACHIEVEMENTS: &[&str] = &[
"Implemented algorithms for a real-time robotics system part of the See and Spray Technology by leveraging C++ 17 and Python in collaboration with the team. Improved log storage and speed efficiency.",
"Created a python tool to automatically sync and upload logs to the AWS cloud, improving the productivity of the team by 40 percent. Provided feature updates as per JIRA requests.",
];

#[rustfmt::skip]
const SJSU_RA_ACHIEVEMENTS: &[&str] = &[
"Successfully developed two embedded prototypes for firefighters in high-risk environments, integrating wireless technologies like LTE, BLE, and various sensors with AWS IoT Core.",
"Achieved a significant 50 USD reduction in firmware costs per device and a 20% improvement in power efficiency for the second prototype.",
];

pub fn info() -> UserInfo {
    let skill = UserSkillInfo {
        skills: &[
            UserOneSkillInfo {
                title: "Programming Languages",
                topics: &["Rust", "C++17", "C", "Python", "Dart"],
            },
            UserOneSkillInfo {
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
            UserOneSkillInfo {
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
            UserOneSkillInfo {
                title: "Hardware Interfaces",
                topics: &["I2C", "SPI", "CAN", "UART"],
            },
            UserOneSkillInfo {
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
    let experience = UserExperienceInfo {
        roles: &[
            UserOneExperienceInfo {
                start: "2023-12",
                end: None,
                company: "Qualcomm Technologies",
                title: "Senior Software Engineer",
                achievements: QUALCOMM_SENIOR_ENGINEER_ACHIEVEMENTS,
                skills: &[
                    "Rust",
                    "C++17",
                    "Parallelism",
                    "Concurrency",
                    "Flatbuffer",
                    "Encryption",
                    "OpenTelemetry",
                    "FFI",
                ],
            },
            UserOneExperienceInfo {
                start: "2021-07",
                end: Some("2023-11"),
                company: "Qualcomm Technologies",
                title: "Cellular Software Engineer",
                achievements: QUALCOMM_ENGINEER_ACHIEVEMENTS,
                skills: &["C", "Python", "LTE", "Jenkins CI", "Sequence Diagrams"],
            },
            UserOneExperienceInfo {
                start: "2020-12",
                end: Some("2021-05"),
                company: "TuringSense",
                title: "Firmware Engineer Intern",
                achievements: TURINGSENSE_INTERN_ACHIEVEMENTS,
                skills: &["C", "NXP Semiconductors", "BLE", "Device Drivers"],
            },
            UserOneExperienceInfo {
                start: "2020-05",
                end: Some("2020-08"),
                company: "Blue River Technology",
                title: "System Software Intern",
                achievements: BLUERIVER_INTERN_ACHIEVEMENTS,
                skills: &["C++17", "Python", "CAN J1939", "Nvidia", "Linux"],
            },
            UserOneExperienceInfo {
                start: "2019-10",
                end: Some("2020-05"),
                company: "San Jose State University",
                title: "Research Assistant",
                achievements: SJSU_RA_ACHIEVEMENTS,
                skills: &["GPS", "LTE", "MBED OS", "Zephyr RTOS", "BG96"],
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

    let resume = UserResumeInfo {
        skill,
        experience,
        education,
    };
    UserInfo { resume }
}
