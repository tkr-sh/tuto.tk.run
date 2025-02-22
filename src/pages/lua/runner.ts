import wasmoon from "wasmoon";

const factory = new wasmoon.LuaFactory()

const buildRunners = () => {
    $(".runner").forEach(runner => {
        console.log(runner);
        let span = $new("header", {}, "Lua runner");
        runner.prep(span)
        runner.add(
            $new(
                "footer",
                {},
                undefined,
                [
                    $new("button", {className: "run lua-button"}, "Run"),
                    $new("button", {className: "open lua-button"}, "Open in editor"),
                ]
            )
        )

        const runBtn = runner.$(".run")[0];

        runBtn.on("click", async () => {
            rmOutput(runner);
            const lua = await factory.createEngine();

            try {
                let s = "";
                lua.global.set('read',  lua.global.get('io').read);
                lua.global.set('random',  lua.global.get('math').random);
                lua.global.set('print', (...args: any[]) => s += args.join("\t") + "\n")
                await lua.doString(runner.$("pre")[0].text())
                addOutput(runner, s);
            } catch (e) {
                addOutput(runner, "" + e);
            } finally {
                lua.global.close()
            }
        });
    });
}

buildRunners();


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
