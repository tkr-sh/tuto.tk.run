import { runLua } from "./run-lua";

const buildRunners = () => {
    $(".runner").forEach(runner => {
        let span = $new("header", {}, "Lua runner");
        let openInEditor = $new("a", {className: "open lua-button", href: "/lua/editor"}, null);
        openInEditor.textContent = "Open in editor";

        runner.prep(span)
        runner.add(
            $new(
                "footer",
                {},
                undefined,
                [
                    $new("button", {className: "run lua-button"}, "Run"),
                    openInEditor,
                ]
            )
        )

        const runBtn = runner.$(".run")[0];

        runBtn.on("click", async () => {
            rmOutput(runner);
            let s = "";

            await runLua(runner.$("pre")[0].text(), {
                onPrint: line => {
                    s += line + "\n";
                    addOutput(runner, s);
                },
                onInput: line => {
                    s += line + "\n";
                },
                onError: err => {
                    addOutput(runner, s + "\n" + err);
                },
            });
        });
    });
}

const rmOutput = (c: HTMLElement) => {
    if (c.children[c.children.length - 1].classList.contains("output")) {
        c.removeChild(c.children[c.children.length - 1])
    }
}
const addOutput = (c: HTMLElement, stdout: string) => {
    rmOutput(c);

    const newOutput = $new(
        "div",
        {className: "output"},
        "",
        stdout.split("\n").map(e => $new("div", {}, e))
    );
    c.add(newOutput)
}
