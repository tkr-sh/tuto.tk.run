use {
    maud::{html, Markup},
    wini_macros::page,
};

#[page]
pub async fn render() -> Markup {
    html! {
        #editor contenteditable="true" {
            div {
                "hey"
            }
        }
        .wrapper-output {
            header {
                button #run.lua-button {
                    "Run"
                }
                button.lua-button {
                    "[]"
                }
            }
            .output {

            }
        }
    }
}
