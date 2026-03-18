use dioxus::prelude::*;

mod daisyui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

//
const CONSTRUCTION_DRAWING: Asset = asset!("/assets/img/construction_drawing.jpg");

#[derive(Routable, Debug, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
}

#[cfg(feature = "server")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Meta { name: "description", content: "coder137.portfolio" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "mx-auto max-w-5xl",
            WebsiteHeader {}
            Router::<Route> {}
            WebsiteFooter {}
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "hero min-h-screen",
            style: "background-image: url({CONSTRUCTION_DRAWING});",
            div { class: "hero-overlay" }
            div { class: "hero-content text-neutral-content text-center",
                div { class: "max-w-md",
                    h1 { class: "mb-5 text-5xl font-bold", "Coming soon!" }
                }
            }
        }
    }
}

#[component]
pub fn WebsiteHeader() -> Element {
    rsx! {
        header {
            daisyui::Navbar { class: "bg-base-200 shadow-sm mt-2",
                daisyui::NavbarStart {
                    a { class: "text-2xl font-black", href: "/", "Niket Naidu" }
                }
                daisyui::NavbarEnd {
                    daisyui::Menu {
                        class: "gap-2",
                        menu_type: daisyui::MenuType::Horizontal,
                        a { href: "https://www.linkedin.com/in/niket-naidu/",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaLinkedin,
                            }
                        }

                        a { href: "https://github.com/coder137",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub,
                            }
                        }

                        a { href: "mailto:niketnaiduus@gmail.com",
                            dioxus_free_icons::Icon {
                                width: 20,
                                height: 20,
                                icon: dioxus_free_icons::icons::fa_brands_icons::FaGoogle,
                            }
                        }
                    }

                }
            }
        }
    }
}

#[component]
pub fn WebsiteFooter() -> Element {
    rsx! {
        daisyui::Footer { center: true, class: "footer-horizontal bg-base-200 p-2",
            aside { class: "grid-flow-col items-center",
                p { class: "text-base", "\u{00A9} 2025 Niket Naidu. All right reserved." }
            }

            nav { class: "grid-flow-col items-center",
                a { href: "https://www.linkedin.com/in/niket-naidu/",
                    dioxus_free_icons::Icon {
                        width: 20,
                        height: 20,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaLinkedin,
                    }
                }

                a { href: "https://github.com/coder137",
                    dioxus_free_icons::Icon {
                        width: 20,
                        height: 20,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaGithub,
                    }
                }

                a { href: "mailto:niketnaiduus@gmail.com",
                    dioxus_free_icons::Icon {
                        width: 20,
                        height: 20,
                        icon: dioxus_free_icons::icons::fa_brands_icons::FaGoogle,
                    }
                }
            }
        }
    }
}
