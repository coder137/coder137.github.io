use std::sync::LazyLock;

use crate::info::*;

fn parse_achievements_data(data: &str) -> Vec<&str> {
    data.split("\n")
        .filter(|d| !d.is_empty())
        .collect::<Vec<_>>()
}

static QUALCOMM_SENIOR_ENGINEER_ACHIEVEMENTS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let data = include_str!("../../assets/info/resume/qualcomm_senior_engineer_achievements.md");
    parse_achievements_data(data)
});

static QUALCOMM_ENGINEER_ACHIEVEMENTS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let data = include_str!("../../assets/info/resume/qualcomm_engineer_achievements.md");
    parse_achievements_data(data)
});

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

#[rustfmt::skip]
const PROJECT_LOWLEVEL_RUST_ABOUT: &str = "Rust on microcontrollers";

#[rustfmt::skip]
const PROJECT_BUILDCC_ABOUT: &str = "Alternative to Makefiles by using the feature rich C++ language instead of a custom DSL.";

#[rustfmt::skip]
const PROJECT_CONNECTED_HEALTHCARE_ABOUT: &str = "Peer to peer mesh based network using Google’s OpenThread framework to monitor large crowds, as well as to collect and forward data to healthcare personnel for further analysis and diagnosis.";

#[rustfmt::skip]
const PROJECT_ENTERPRISE_FIRMWARE_ABOUT: &str = "Enterprise-level firmware stack from scratch using the GCC ARM toolchain";

pub fn resume() -> UserResumeInfo {
    let skills = UserSkillInfo {
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

    static QUALCOMM_ROLES: LazyLock<Vec<UserOneExperienceTitleInfo>> = LazyLock::new(|| {
        vec![
            UserOneExperienceTitleInfo {
                title: "Senior Software Engineer",
                start: (2023, 12),
                end: None,
                achievements: &QUALCOMM_SENIOR_ENGINEER_ACHIEVEMENTS,
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
            UserOneExperienceTitleInfo {
                title: "Cellular Software Engineer",
                start: (2021, 7),
                end: Some((2023, 11)),
                achievements: &QUALCOMM_ENGINEER_ACHIEVEMENTS,
                skills: &["C", "Python", "LTE", "Jenkins CI", "Sequence Diagrams"],
            },
        ]
    });

    static ROLES: LazyLock<Vec<UserOneExperienceInfo>> = LazyLock::new(|| {
        vec![
            UserOneExperienceInfo::Many {
                company: "Qualcomm Technologies",
                titles: &QUALCOMM_ROLES,
            },
            UserOneExperienceInfo::One {
                company: "TuringSense",
                title: UserOneExperienceTitleInfo {
                    title: "Firmware Engineer Intern",
                    start: (2020, 12),
                    end: Some((2021, 5)),
                    achievements: TURINGSENSE_INTERN_ACHIEVEMENTS,
                    skills: &["C", "NXP Semiconductors", "BLE", "Device Drivers"],
                },
            },
            UserOneExperienceInfo::One {
                company: "Blue River Technology",
                title: UserOneExperienceTitleInfo {
                    title: "System Software Intern",
                    start: (2020, 5),
                    end: Some((2020, 8)),
                    achievements: BLUERIVER_INTERN_ACHIEVEMENTS,
                    skills: &["C++17", "Python", "CAN J1939", "Nvidia", "Linux"],
                },
            },
            UserOneExperienceInfo::One {
                company: "San Jose State University",
                title: UserOneExperienceTitleInfo {
                    title: "Research Assistant",
                    start: (2019, 10),
                    end: Some((2020, 5)),
                    achievements: SJSU_RA_ACHIEVEMENTS,
                    skills: &["GPS", "LTE", "MBED OS", "Zephyr RTOS", "BG96"],
                },
            },
        ]
    });

    let experience = UserExperienceInfo { roles: &ROLES };

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

    let projects = UserProjectInfo {
        projects: &[
            UserOneProjectInfo {
                start: (2023, 1),
                end: Some((2023, 4)),
                title: "Lowlevel Rust",
                link: Some("https://github.com/coder137/lowlevel_rust"),
                about: PROJECT_LOWLEVEL_RUST_ABOUT,
                achievements: &[],
                skills: &[],
                tags: &["Personal", "Rust", "Firmware"],
            },
            UserOneProjectInfo {
                start: (2021, 2),
                end: Some((2022, 12)),
                title: "Build in CPP [BuildCC]",
                link: Some("https://github.com/coder137/build_in_cpp"),
                about: PROJECT_BUILDCC_ABOUT,
                achievements: &[],
                skills: &[],
                tags: &["Personal", "C++", "Buildsystem"],
            },
            // UserOneProjectInfo {
            //     start: (2020, 8),
            //     end: Some((2021, 5)),
            //     title: "Connected and Distributed Sensing System for Healthcare",
            //     link: Some("https://github.com/Connected-Healthcare"),
            //     about: PROJECT_CONNECTED_HEALTHCARE_ABOUT,
            //     achievements: &[],
            //     skills: &[],
            //     tags: &["University", "Master's Project"],
            // },
            UserOneProjectInfo {
                start: (2019, 12),
                end: Some((2021, 1)),
                title: "Enterprise Firmware platform development",
                link: Some("https://github.com/coder137/STM32-Repo"),
                about: PROJECT_ENTERPRISE_FIRMWARE_ABOUT,
                achievements: &[],
                skills: &[],
                tags: &["Personal", "Firmware"],
            },
        ],
    };

    UserResumeInfo {
        skills,
        experience,
        education,
        projects,
    }
}
