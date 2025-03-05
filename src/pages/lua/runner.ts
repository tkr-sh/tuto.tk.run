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
                    s += input  + "\n";
                    return input
                });

                let io = lua.global.get("io");
                io.read = () => {
                    console.log("lolilol")
                    let input = prompt("Input:");
                    s += input  + "\n";
                    return input
                }
                lua.global.set("io", io);
                lua.global.set('random',  lua.global.get('math').random);
                lua.global.set('print', (...args: any[]) => {
                    s += args.join("\t") + "\n";
                    alert(args.join("\t"));
                    addOutput(runner, s);
                });
                await lua.doString(runner.$("pre")[0].text());
                addOutput(runner, s);
            } catch (e) {
                addOutput(runner, s + "\n" + e);
            } finally {
                lua.global.close()
            }
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
