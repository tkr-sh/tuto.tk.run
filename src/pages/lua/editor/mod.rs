use {
    maud::{html, Markup},
    wini_macros::page,
};

const DEFAULT_CODE: &str = r#"-- Welcome to TK's Lua editor!
-- Write whatever you want and hit "Run".

print("Hello, world!")

for i = 1, 5 do
    print("Counting: " .. i)
end
"#;

#[page(title = "TK's Lua editor")]
pub async fn render() -> Markup {
    html! {
        #editor {
            header #editor-header {
                a #editor-logo href="/lua" {
                    span #lua-title { "Lua" }
                    " editor"
                }
                .spacer {}
                #editor-actions {
                    button #share.lua-button.ghost
                        data-on:click="navigator.clipboard.writeText(window.location.href); $copied = true"
                        "data-on:click__delay.2s"="$copied = false"
                        data-text="$copied ? 'Copied!' : 'Share'"
                    {
                        "Share"
                    }
                    button #run.lua-button {
                        "Run"
                    }
                }
            }
            #editor-body {
                section #pane-code.pane {
                    .pane-header {
                        span.pane-title { "main.lua" }
                    }
                    .pane-content {
                        .editor-wrap {
                            pre #highlight aria-hidden="true" {}
                            textarea #code
                                spellcheck="false"
                                autocomplete="off"
                                autocapitalize="off"
                                wrap="off" {
                                (DEFAULT_CODE)
                            }
                        }
                    }
                }
                .gutter #gutter {}
                section #pane-output.pane {
                    .pane-header {
                        span.pane-title { "Output" }
                        button #clear.ghost { "Clear" }
                    }
                    .pane-content {
                        #output {
                            .placeholder { "Press \"Run\" to execute your code." }
                        }
                    }
                }
            }
            footer #editor-statusbar {
                span #status { "Ready" }
                .spacer {}
                span #cursor-pos { "Ln 1, Col 1" }
            }
        }
    }
}
