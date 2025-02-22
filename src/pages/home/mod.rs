use {
    maud::{html, Markup},
    wini_macros::page,
};

#[page]
pub async fn render() -> Markup {
    html! {
        h1 { "TK's Tutorials" }
        ul {
            li {
                a href="/lua" {
                    img src="https://upload.wikimedia.org/wikipedia/commons/c/cf/Lua-Logo.svg" ;
                    "Lua"
                }
            }
        }
    }
}
