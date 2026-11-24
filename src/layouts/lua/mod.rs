use {
    crate::{pages::lua::PAGES_STRUCTURE, utils::language::Language},
    axum::http::Uri,
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(uri: Uri, child: Markup) -> Markup {
    let language = Language::from_path(uri.path()).unwrap_or(Language::English);

    html! {
        nav
            #sidebar
            dir=(if language == Language::Arabic { "rtl" } else { "ltr" })
            data-signals:barHidden="false"
            data-class:hidden = "$barHidden"
            data-on:click__outside = "if (window.innerWidth < 1200) $barHidden = true"
        {
            button
                #hide-sidebar
                data-on:click="$barHidden = !$barHidden"
                data-text="$barHidden ? '>' : '<'"
            {}
            (PAGES_STRUCTURE.rec_display(&language))
        }
        main #main data-on:pushstate__window="console.log('hey')" {
            (child)
        }
    }
}
