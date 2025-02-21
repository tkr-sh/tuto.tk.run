use {
    crate::{
        components::notfound,
        utils::tree::{compute_pages, PageOrDirectory},
    },
    axum::extract::Request,
    maud::{html, Markup, PreEscaped},
    std::{collections::HashMap, sync::LazyLock},
    wini_macros::{cache, page},
};

pub static PAGES_STRUCTURE: LazyLock<PageOrDirectory> =
    LazyLock::new(|| ron::from_str(&include_str!("./structure.ron")).unwrap());

pub static PAGES_BY_PATH: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| compute_pages("./src/pages/lua"));

pub static PAGE_TITLE_BY_PATH: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| PAGES_STRUCTURE.rec_get_page_titles());


#[cache]
#[page]
pub async fn render(req: Request) -> Markup {
    let requested_page = req
        .uri()
        .path()
        .split('/')
        .skip(2)
        .next()
        .unwrap_or("introduction");

    let Some(result) = PAGES_BY_PATH.get(requested_page) else {
        return html! { [notfound::render] };
    };

    println!("{PAGE_TITLE_BY_PATH:#?}");

    let (previous_page, next_page) = PAGES_STRUCTURE.get_nearest_pages(requested_page);


    html! {
        nav #sidebar {
            (PAGES_STRUCTURE.rec_display())
        }
        main #main {
            header {
                button #hide-sidebar {
                    "uwu"

                }
                h1 {
                    "TK's "
                    span #lua-title {
                        "Lua"
                    }
                    " tutorial"
                }
                .spacer {}
                h2 {
                    (PAGE_TITLE_BY_PATH.get(requested_page).as_deref().unwrap_or_else(|| &""))
                }
            }
            main {
                #content {
                    (PreEscaped(result))
                }
            }
            footer.buttons-previous-next {
                @if let Some(previous_page) = previous_page {
                    button.previous
                        hx-on-click={"setHlPage('"(previous_page)"')"}
                        hx-get={"/htmx/" (previous_page)}
                        hx-target="body"
                        hx-replace-url={"/lua/" (previous_page)}
                    {
                        "Previous"
                    }
                } @else {
                    div .placeholder-previous-next {}
                }
                @if let Some(next_page) = next_page {
                    button.next
                        hx-get={"/htmx/" (next_page)}
                        hx-replace-url={"/lua/" (next_page)}
                        hx-target="body"
                        hx-on-click={"setHlPage('"(next_page)"')"}
                    {
                        "Next"
                    }
                } @else {
                    .placeholder-previous-next {}
                }
            }
        }
    }
}
