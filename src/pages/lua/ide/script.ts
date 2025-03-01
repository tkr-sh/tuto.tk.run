import wasmoon from "wasmoon";
import {parse} from "luaparse";

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

const TOKENS = {
    EOF: 1,
    STRING: 2,
    KEYWORD: 4,
    IDENT: 8,
    NUM: 16,
    PUNCT: 32,
    BOOL: 64,
    NIL: 128,
    VARARG: 256,
};

const colorize = () => {
    const editor = $("#editor");
    const currentCode = editor.text();
    const cursorPos = getCursor();
    console.log(cursorPos);
    console.log(editor);
    console.log(editor.getAttribute("selectionStart"));

    const lexer = parse(currentCode, { wait: true });
    console.log(lexer, currentCode);
    let totalString = "";
    let lastIndex = 0;

    for (;;) {
        const token = lexer.lex();

        if (!token || token.type === TOKENS.EOF) {
            break;
        }

        totalString += currentCode.slice(lastIndex, token.range[0])
        totalString += "<div>"
        totalString += currentCode.slice(token.range[0], token.range[1])
        totalString += "</div>"


        lastIndex = token.range[1];
    }

    totalString += currentCode.slice(lastIndex, -1)
    console.log(totalString);


    setCursor(cursorPos);
    editor.innerHTML = currentCode;
}

setInterval(colorize, 4000);



const getCursor = (): number => {
    const selection = window.getSelection();
    const range = selection.getRangeAt(0);
    const clonedRange = range.cloneRange();
    clonedRange.selectNodeContents($("#editor"));
    clonedRange.setEnd(range.endContainer, range.endOffset);

    return clonedRange.toString().length;
}

const setCursor = (pos: number) => {
    let el = $("#editor");

    let setpos = document.createRange();
    let set = window.getSelection();

    setpos.setStart(el.childNodes[0], pos);
    setpos.collapse(true);
    set.removeAllRanges();
    set.addRange(setpos);

    el.focus();
}
