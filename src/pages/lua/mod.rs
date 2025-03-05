pub mod ide;
use {
    crate::{
        components::notfound,
        utils::{
            language::Language,
            tree::{compute_pages, PageOrDirectory, Title},
        },
    },
    axum::extract::Request,
    maud::{html, Markup, PreEscaped},
    std::{collections::HashMap, sync::LazyLock},
    wini_macros::{cache, page},
};

pub static PAGES_STRUCTURE: LazyLock<PageOrDirectory> =
    LazyLock::new(|| ron::from_str(&include_str!("./structure.ron")).unwrap());

pub static PAGES_BY_PATH: LazyLock<HashMap<(String, Language), String>> =
    LazyLock::new(|| compute_pages("./src/pages/lua"));

pub static PAGE_TITLE_BY_PATH: LazyLock<HashMap<&str, &Title>> =
    LazyLock::new(|| PAGES_STRUCTURE.rec_get_page_titles());

// #[cache]
#[page(title = "TK's Lua tutorial")]
pub async fn render(req: Request) -> Markup {
    let requested_page = req
        .uri()
        .path()
        .split('/')
        .skip(2)
        .next()
        .unwrap_or("introduction");

    #[cfg(feature = "random-lang")]
    let language = Language::French;
    #[cfg(not(feature = "random-lang"))]
    let Ok(language) = Language::try_from(&req) else {
        return html! { [notfound::render] };
    };


    let Some(result) = PAGES_BY_PATH.get(&(requested_page.to_owned(), language)) else {
        return html! { [notfound::render] };
    };

    let (previous_page, next_page) = PAGES_STRUCTURE.get_nearest_pages(requested_page);

    html! {
        header {
            h1 {
                "TK's "
                span #lua-title {
                    "Lua"
                }
                " tutorial"
            }
            .spacer {}
            h2 {
                (PAGE_TITLE_BY_PATH.get(requested_page).map(|title| title.str_by_language(&language)).unwrap_or_else(|| &""))
            }
            #flags {
                button _="on click toggle .visible on #flags" {
                    img src="/lang.svg";
                }
                ul {
                    @for lang in ["en", "es", "pt", "fr", "de"] {
                        li {
                            a href=(format!("https://{lang}.tuto.tk.run/lua/{requested_page}")) {
                                img src=(format!("/flag/{lang}.svg"));
                            }
                        }
                    }
                }
            }
        }
        main _=(PreEscaped("on load init js onNewPage() end then \
                go to top of previous <header/>")){
            #content class=(requested_page) {
                (PreEscaped(result))
            }
        }
        footer.buttons-previous-next {
            @if let Some(previous_page) = previous_page {
                button.previous.lua-button
                    hx-get={"/htmx/" (previous_page)}
                    hx-target="#main"
                    hx-replace-url={"/lua/" (previous_page)}
                {
                    "Previous"
                }
            } @else {
                div .placeholder-previous-next {}
            }
            @if let Some(next_page) = next_page {
                button.next.lua-button
                    hx-get={"/htmx/" (next_page)}
                    hx-target="#main"
                    hx-replace-url={"/lua/" (next_page)}
                {
                    "Next"
                }
            } @else {
                .placeholder-previous-next {}
            }
        }
    }
}
