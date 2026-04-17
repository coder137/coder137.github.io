use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

mod daisyui;
mod ui;

mod info;
mod website;

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
            website::Header {}
            main { Router::<website::Route> {} }
            website::Footer {}
        }
    }
}
