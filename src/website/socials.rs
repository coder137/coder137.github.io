use crate::*;

#[component]
pub fn Socials() -> Element {
    rsx! {
        div { class: "flex flex-row flex-wrap gap-2",
            a {
                href: "https://www.linkedin.com/in/niket-naidu/",
                aria_label: "Linkedin link",
                dioxus_free_icons::Icon {
                    width: 20,
                    height: 20,
                    icon: dioxus_free_icons::icons::fa_brands_icons::FaLinkedin,
                }
            }

            a {
                href: "https://github.com/coder137",
                aria_label: "Github link",
                dioxus_free_icons::Icon {
                    width: 20,
                    height: 20,
                    icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub,
                }
            }

            a {
                href: "mailto:niketnaiduus@gmail.com",
                aria_label: "Email link",
                dioxus_free_icons::Icon {
                    width: 20,
                    height: 20,
                    icon: dioxus_free_icons::icons::fa_solid_icons::FaEnvelope,
                }
            }
        }
    }
}
