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
            let s = "";
            const lua = await factory.createEngine();

            try {
                lua.global.set('read', () => {
                    let input = prompt("Input:");
                    console.log(input, s);
                    s += input  + "\n";
                    return input
                });
                lua.global.set('random',  lua.global.get('math').random);
                lua.global.set('print', (...args: any[]) => {
                    console.log("uwuwuu")
                    s += args.join("\t") + "\n";
                    console.log(s)
                    addOutput(runner, s);
                });
                // await lua.doString(runner.$("pre")[0].text())
                await lua.doString(runner.$("pre")[0].text())
                addOutput(runner, s);
            } catch (e) {
                addOutput(runner, s + "\n" + e);
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
    console.log(c);

    const newOutput = $new(
        "div",
        {className: "output"},
        "",
        stdout.split("\n").map(e => $new("div", {}, e))
    );
    console.log(newOutput);
    c.add(newOutput)
}
