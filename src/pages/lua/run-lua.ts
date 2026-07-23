import wasmoon from "wasmoon";

// Shared Lua runtime for the tutorial runners (runner.ts) and the editor.
// Callers decide how to display output via the handlers.
type LuaRunHandlers = {
    onPrint: (text: string) => void;
    onInput?: (text: string) => void;
    onError?: (error: string) => void;
};

const luaFactory = new wasmoon.LuaFactory();

const runLua = async (code: string, handlers: LuaRunHandlers): Promise<void> => {
    const lua = await luaFactory.createEngine();

    try {
        const read = () => {
            const input = prompt("Input:") ?? "";
            handlers.onInput?.(input);
            return input;
        };
        lua.global.set("read", read);

        const io = lua.global.get("io");
        if (io) {
            io.read = read;
            lua.global.set("io", io);
        }

        lua.global.set("random", lua.global.get("math").random);
        lua.global.set("print", (...args: any[]) => {
            const line = args.map(a => `${a}`).join("\t");
            alert(line);
            handlers.onPrint(line);
        });

        await lua.doString(code);
    } catch (e) {
        if (handlers.onError) handlers.onError(`${e}`);
        else throw e;
    } finally {
        lua.global.close();
    }
};
