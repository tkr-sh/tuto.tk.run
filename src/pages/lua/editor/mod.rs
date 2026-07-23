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
                        _="on click \
                            writeText(window.location.href) on navigator.clipboard \
                            then put 'Copied!' into me \
                            then wait 1.5s \
                            then put 'Share' into me" {
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
