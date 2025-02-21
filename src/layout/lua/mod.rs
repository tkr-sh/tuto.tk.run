use {
    crate::pages::lua::PAGES_STRUCTURE,
    maud::{html, Markup},
    wini_macros::layout,
};

#[layout]
pub async fn render(child: &str) -> Markup {
    html! {
        nav #sidebar {
            (PAGES_STRUCTURE.rec_display())
        }
        main #main {
            (child)
        }
    }
}
