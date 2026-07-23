use {
    crate::{pages::lua::PAGES_STRUCTURE, utils::language::Language},
    axum::extract::Request,
    maud::{html, Markup, PreEscaped},
    std::str::FromStr,
    wini_macros::layout,
};

#[layout]
pub async fn render(child: Markup) -> Markup {
    // TODO
    let language = Language::English;

    html! {
        nav #sidebar data-signals:barHidden="false"  data-class:hidden = "$barHidden" data-on:click__outside="if (window.innerWidth < 1200) $barHidden = true"{
            button #hide-sidebar data-on:click="$barHidden = !$barHidden" data-text="$barHidden ? '>' : '<'"
            {
                "<"
            }
            (PAGES_STRUCTURE.rec_display(&language))
        }
        main #main {
            (child)
        }
    }
}
