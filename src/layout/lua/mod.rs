use {
    crate::{pages::lua::PAGES_STRUCTURE, utils::language::Language},
    axum::extract::Request,
    maud::{html, Markup, PreEscaped},
    std::str::FromStr,
    wini_macros::layout,
};

#[layout]
pub async fn render(child: &str) -> Markup {
    // TODO
    let language = Language::English;

    html! {
        script type="text/hyperscript" {
            (PreEscaped("def liClick() \
                if the innerWidth of the window < 1200 then \
                    add .hidden to #sidebar \
                    put '>' into #hide-sidebar \
                end \
           end"))
        }
        nav #sidebar {
            button #hide-sidebar _="on click \
                toggle .hidden on #sidebar \
                if #sidebar matches .hidden then put '>' into me else put '<' into me" {
                "<"
            }
            (PAGES_STRUCTURE.rec_display(&language))
        }
        main #main {
            (PreEscaped(child))
        }
    }
}
