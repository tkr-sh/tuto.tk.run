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

pub static PAGES: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| compute_pages("./src/pages/lua"));


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

    let Some(result) = PAGES.get(requested_page) else {
        return html! { [notfound::render] };
    };

    let (previous_page, next_page) = PAGES_STRUCTURE.get_nearest_pages(requested_page);


    html! {
        header {
            "TK's lua tutorial"
        }
        @if let Some(previous_page) = previous_page {
            button.previous-next
                hx-on-click={"setHlPage('"(previous_page)"')"}
                hx-get={"/htmx/" (previous_page)}
                hx-target="#horizontal-content"
                hx-replace-url={"/doc/" (previous_page)}
            {
                // (PreEscaped(
                //         svg(Type::Solid, "angle-left"
                //
                //         ).unwrap()))
            }
        } @else {
            div .placeholder-previous-next {}
        }
        main {
            #content {
                (PreEscaped(result))
            }
        }
        @if let Some(next_page) = next_page {
            button.previous-next
                hx-get={"/htmx/" (next_page)}
                hx-replace-url={"/doc/" (next_page)}
                hx-target="#horizontal-content"
                hx-on-click={"setHlPage('"(next_page)"')"}
            {
                // (PreEscaped(
                //     svg(
                //         Type::Solid,
                //         "angle-right"
                //     )
                //     .unwrap()
                // ))
            }
        } @else {
            .placeholder-previous-next {}
        }
    }
}
