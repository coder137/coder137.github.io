pub use crate::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        daisyui::Footer {
            class: "footer-horizontal bg-base-200 rounded-field flex justify-between items-center p-2",
            center: false,
            p { class: "text-base", "\u{00A9} 2026 Niket Naidu. All rights reserved." }
            website::Socials {}
        }
    }
}
