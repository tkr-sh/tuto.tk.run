import { encodeCodeToUrl } from "./code-url";
import { runLua } from "./run-lua";

type OutLine = { text: string; cls?: string };

const buildRunners = () => {
    $(".runner").forEach(runner => {
        const code = runner.$("pre")[0].text();
        let span = $new("header", {}, "Lua runner");
        let openInEditor = $new(
            "a",
            {className: "open lua-button", href: `/lua/editor#code=${encodeCodeToUrl(code)}`},
            null
        );
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
            const lines: OutLine[] = [];

            const push = (text: string, cls?: string) => {
                text.split("\n").forEach(t => lines.push({text: t, cls}));
                addOutput(runner, lines);
            };

            await runLua(runner.$("pre")[0].text(), {
                onPrint: line => push(line),
                onInput: line => push(line, "input"),
                onError: err => push(err, "err"),
            });
        });
    });
}

const rmOutput = (c: HTMLElement) => {
    if (c.children[c.children.length - 1].classList.contains("output")) {
        c.removeChild(c.children[c.children.length - 1])
    }
}
const addOutput = (c: HTMLElement, lines: OutLine[]) => {
    rmOutput(c);

    const newOutput = $new(
        "div",
        {className: "output"},
        "",
        lines.map(line => $new("div", {className: line.cls ?? ""}, line.text))
    );
    c.add(newOutput)
}
