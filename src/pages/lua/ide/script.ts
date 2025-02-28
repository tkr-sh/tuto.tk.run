import wasmoon from "wasmoon";

const factory = new wasmoon.LuaFactory()

$("#run").on("click", async () => {
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

})
