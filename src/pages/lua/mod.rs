use {
    crate::{
        components::notfound,
        utils::{
            language::Language,
            tree::{compute_pages, PageOrDirectory, Title},
            wini::buffer::buffer_to_string,
        },
    },
    async_stream::stream,
    axum::{
        extract::Request,
        response::{IntoResponse, Response, Sse},
    },
    datastar::{consts::ElementPatchMode, prelude::PatchElements},
    itertools::Itertools,
    maud::{html, Markup, PreEscaped},
    std::{collections::HashMap, convert::Infallible, sync::LazyLock},
    wini_macros::page,
};

pub mod editor;

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
            #flags data-class:visible="$showFlags" {
                button data-on:click="$showFlags = !$showFlags" {
                    img src="/lang.svg";
                }
                ul {
                    @for lang in ["en", "es", "pt", "fr", "de"] {
                        li {
                            a href=(format!("https://tuto.tk.run/{lang}/lua/{requested_page}")) {
                                img src=(format!("/flag/{lang}.svg"));
                            }
                        }
                    }
                }
            }
        }
        main
            data-init="const f = () => {if (typeof onNewPage !== 'undefined') { onNewPage() } else {setTimeout(f, 10)} }; f()"
        {
            #content class=(requested_page) {
                (PreEscaped(result))
            }
        }
        footer.buttons-previous-next {
            @if let Some(previous_page) = previous_page {
                button.previous.lua-button
                    data-on:click={"@get('/ds/lua/"(previous_page)"')"}
                {
                    "Previous"
                }
            } @else {
                div .placeholder-previous-next {}
            }
            @if let Some(next_page) = next_page {
                button.next.lua-button
                    data-on:click={"@get('/ds/lua/"(next_page)"')"}
                {
                    "Next"
                }
            } @else {
                .placeholder-previous-next {}
            }
        }
    }
}


pub async fn datastar(req: Request) -> Response {
    let uri_without_ds = {
        let mut uri = req.uri().path().split('/').skip(1).join("/");
        uri.insert(0, '/');
        uri
    };

    let resp = render(req).await;

    Sse::new(stream! {
        // Body
        let patch = PatchElements::new(buffer_to_string(resp).await.unwrap_or_default())
            .selector("#main")
            .mode(ElementPatchMode::Inner);
        yield Ok::<_, Infallible>(patch.write_as_axum_sse_event());

        let json_uri =
            serde_json::to_string(&uri_without_ds).unwrap_or_else(|_| "\"/\"".to_owned());
        let patch = PatchElements::new(format!(
            "<script>window.history.pushState({{urlPath:{json_uri}}},\"\",{json_uri});</script>"
        ))
        .selector("head")
        .mode(ElementPatchMode::Append);
        yield Ok(patch.write_as_axum_sse_event());

        let patch = PatchElements::new(r"<script>f = (n) => {if (typeof onNewPage !== 'undefined' || n > 1000) { onNewPage() } else {setTimeout(() => f(n+1), 10)} };f(0)</script>")
            .selector("head")
        .mode(ElementPatchMode::Append);
        yield Ok(patch.write_as_axum_sse_event());

        let patch = PatchElements::new("<script>document.getElementById('main').scrollTo(0,0)</script>").selector("head")
            .mode(ElementPatchMode::Append);
        yield Ok(patch.write_as_axum_sse_event());


    })
    .into_response()
}
