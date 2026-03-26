use crate::info::*;

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

#[rustfmt::skip]
const PROJECT_CONNECTED_HEALTHCARE_ABOUT: &str = "To create a peer to peer mesh based network using Google’s OpenThread framework to monitor large crowds, as well as to collect and forward data to healthcare personnel for further analysis and diagnosis.";

#[rustfmt::skip]
const PROJECT_ENTERPRISE_FIRMWARE_ABOUT: &str = "To create an enterprise-level firmware stack from scratch using the GCC ARM toolchain";

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

    const Q1: UserOneExperienceTitleInfo = UserOneExperienceTitleInfo {
        title: "Senior Software Engineer",
        start: (2023, 12),
        end: None,
        // TODO, Use DateTime from the chrono library
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
    };
    const Q2: UserOneExperienceTitleInfo = UserOneExperienceTitleInfo {
        title: "Cellular Software Engineer",
        start: (2021, 7),
        end: Some((2023, 11)),
        achievements: QUALCOMM_ENGINEER_ACHIEVEMENTS,
        skills: &["C", "Python", "LTE", "Jenkins CI", "Sequence Diagrams"],
    };
    const QUALCOMM: UserOneExperienceInfo = UserOneExperienceInfo::Group {
        company: "Qualcomm Technologies",
        titles: &[Q1, Q2],
    };

    let experience = UserExperienceInfo {
        roles: &[
            QUALCOMM,
            UserOneExperienceInfo::Individual {
                company: "TuringSense",
                title: UserOneExperienceTitleInfo {
                    title: "Firmware Engineer Intern",
                    start: (2020, 12),
                    end: Some((2021, 5)),
                    achievements: TURINGSENSE_INTERN_ACHIEVEMENTS,
                    skills: &["C", "NXP Semiconductors", "BLE", "Device Drivers"],
                },
            },
            UserOneExperienceInfo::Individual {
                company: "Blue River Technology",
                title: UserOneExperienceTitleInfo {
                    title: "System Software Intern",
                    start: (2020, 5),
                    end: Some((2020, 8)),
                    achievements: BLUERIVER_INTERN_ACHIEVEMENTS,
                    skills: &["C++17", "Python", "CAN J1939", "Nvidia", "Linux"],
                },
            },
            UserOneExperienceInfo::Individual {
                company: "San Jose State University",
                title: UserOneExperienceTitleInfo {
                    title: "Research Assistant",
                    start: (2019, 10),
                    end: Some((2020, 5)),
                    achievements: SJSU_RA_ACHIEVEMENTS,
                    skills: &["GPS", "LTE", "MBED OS", "Zephyr RTOS", "BG96"],
                },
            },
        ],
    };

    let projects = UserProjectInfo {
        projects: &[
            UserOneProjectInfo {
                start: (2020, 8),
                end: Some((2021, 5)),
                title: "Connected and Distributed Sensing System for Healthcare",
                link: Some("https://github.com/Connected-Healthcare"),
                about: PROJECT_CONNECTED_HEALTHCARE_ABOUT,
                achievements: &[],
                skills: &[],
                project_type_tag: "Master's Project",
                project_type: UserOneProjectType::University,
            },
            UserOneProjectInfo {
                start: (2019, 12),
                end: Some((2021, 1)),
                title: "Enterprise Firmware platform development",
                link: Some("https://github.com/coder137/STM32-Repo"),
                about: PROJECT_ENTERPRISE_FIRMWARE_ABOUT,
                achievements: &[],
                skills: &[],
                project_type_tag: "Firmware",
                project_type: UserOneProjectType::Personal,
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

    UserResumeInfo {
        skills,
        experience,
        projects,
        education,
    }
}
