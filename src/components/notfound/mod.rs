use {
    maud::{html, Markup},
    wini_macros::component,
};

#[component]
pub async fn render() -> Markup {
    html! {
        main #not-found {
            h1 { "Not found!" }
            a href="/lua/introduction" {
                "Go to main page"
            }
        }
    }
}
