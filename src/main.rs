use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

mod daisyui;

mod info;
mod navigation;

mod ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }

    // https://dioxuslabs.com/learn/0.7/essentials/fullstack/static_site_generation/
    #[cfg(feature = "server")]
    {
        dioxus::LaunchBuilder::new()
            // Set the server config only if we are building the server target
            .with_cfg(server_only! {
                ServeConfig::builder()
                    // Enable incremental rendering
                    .incremental(
                        dioxus::server::IncrementalRendererConfig::new()
                            // Store static files in the public directory where other static assets like wasm are stored
                            .static_dir(
                                std::env::current_exe()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .join("public")
                            )
                            // Don't clear the public folder on every build. The public folder has other files including the wasm
                            // binary and static assets required for the app to run
                            .clear_cache(false)
                    )
                    .enable_out_of_order_streaming()
            })
            .launch(App);
    }
}

#[component]
fn App() -> Element {
    use_effect(|| {
        let html_element = web_sys::window()
            .expect("Window not found")
            .document()
            .expect("Document not found")
            .document_element()
            .expect("Document element not found")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Failed to cast to HtmlElement");
        html_element.set_lang("en");
        html_element.set_class_name("scroll-smooth");
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Meta {
            name: "description",
            content: "Discover Niket Naidu, an Embedded System Engineer, specializing in real-time systems.",
        }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        WebsitePage {}
    }
}

#[component]
pub fn WebsitePage() -> Element {
    rsx! {
        div { class: "mx-auto max-w-5xl p-2",
            WebsiteHeader {}
            main { Router::<navigation::Route> {} }
            WebsiteFooter {}
        }
    }
}

#[component]
pub fn WebsiteHeader() -> Element {
    let home_route = navigation::Route::Home.to_string();
    rsx! {
        header {
            daisyui::Navbar { class: "bg-base-200 rounded-field",
                daisyui::NavbarStart {
                    a {
                        class: "font-barrio tracking-wide text-4xl",
                        href: home_route,
                        "Niket Naidu"
                    }
                }
                daisyui::NavbarEnd { WebsiteSocials {} }
            }
        }
    }
}

#[component]
pub fn WebsiteFooter() -> Element {
    rsx! {
        daisyui::Footer {
            class: "footer-horizontal bg-base-200 rounded-field flex justify-between items-center p-2",
            center: false,
            p { class: "text-base", "\u{00A9} 2025 Niket Naidu. All rights reserved." }
            WebsiteSocials {}
        }
    }
}

#[component]
pub fn WebsiteSocials() -> Element {
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
