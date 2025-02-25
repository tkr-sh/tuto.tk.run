use {
    crate::{pages::lua::PAGES_STRUCTURE, utils::language::Language},
    axum::extract::Request,
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(child: &str, language: Language) -> Markup {
    html! {
        nav #sidebar {
            button #hide-sidebar {
                "<"
            }
            (PAGES_STRUCTURE.rec_display(&language))
        }
        main #main {
            (PreEscaped(child))
        }
    }
}
