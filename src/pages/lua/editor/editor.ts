// Absolute path: the build doesn't attach cross-folder relative imports.
import { runLua } from "/src/pages/lua/run-lua.js";

const LUA_KEYWORDS = new Set([
    "and", "break", "do", "else", "elseif", "end", "for", "function", "goto",
    "if", "in", "local", "not", "or", "repeat", "return", "then", "until", "while",
]);

const LUA_CONSTANTS = new Set(["nil", "true", "false"]);

const LUA_BUILTINS = new Set([
    "print", "read", "random", "io", "os", "math", "string", "table",
    "pairs", "ipairs", "next", "select", "tonumber", "tostring", "type",
    "pcall", "xpcall", "error", "assert", "setmetatable", "getmetatable",
    "rawget", "rawset", "rawequal", "rawlen", "require",
]);

const escapeHtml = (s: string): string =>
    s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

const wrap = (cls: string, text: string): string =>
    `<span class="${cls}">${escapeHtml(text)}</span>`;

const highlightLua = (src: string): string => {
    const re =
        /(--\[\[[\s\S]*?\]\]|--[^\n]*)|(\[\[[\s\S]*?\]\]|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')|(\b0[xX][0-9a-fA-F]+\b|\b\d+\.?\d*\b)|([A-Za-z_]\w*)/g;

    let out = "";
    let last = 0;
    let m: RegExpExecArray | null;

    while ((m = re.exec(src)) !== null) {
        out += escapeHtml(src.slice(last, m.index));

        const [full, comment, str, num, ident] = m;
        if (comment !== undefined) {
            out += wrap("tok-comment", full);
        } else if (str !== undefined) {
            out += wrap("tok-string", full);
        } else if (num !== undefined) {
            out += wrap("tok-number", full);
        } else if (ident !== undefined) {
            if (LUA_KEYWORDS.has(ident)) out += wrap("tok-keyword", ident);
            else if (LUA_CONSTANTS.has(ident)) out += wrap("tok-constant", ident);
            else if (LUA_BUILTINS.has(ident)) out += wrap("tok-builtin", ident);
            else out += escapeHtml(ident);
        }

        last = re.lastIndex;
    }

    out += escapeHtml(src.slice(last));

    // Extra newline: a <pre> drops one trailing newline, a <textarea> doesn't.
    return out + "\n";
};

const renderHighlight = () => {
    const ta = $("#code") as HTMLTextAreaElement;
    const hl = $("#highlight");
    if (!ta || !hl) return;
    hl.innerHTML = highlightLua(ta.value);
};

const syncScroll = () => {
    const ta = $("#code") as HTMLTextAreaElement;
    const hl = $("#highlight");
    if (!ta || !hl) return;
    hl.scrollTop = ta.scrollTop;
    hl.scrollLeft = ta.scrollLeft;
};

const setStatus = (text: string, kind: "" | "running" | "error" = "") => {
    const status = $("#status");
    if (!status) return;
    status.text(text);
    status.className = kind;
};

const clearOutput = () => {
    const output = $("#output");
    output.innerHTML = "";
};

const appendLine = (text: string, cls: string = "line") => {
    const output = $("#output");
    text.split("\n").forEach(line => {
        output.add($new("div", { className: cls }, line));
    });
};

const runCode = async () => {
    const code = ($("#code") as HTMLTextAreaElement).value;

    clearOutput();
    setStatus("Running...", "running");

    try {
        await runLua(code, {
            onPrint: line => appendLine(line),
            onInput: line => appendLine(line, "meta"),
        });

        setStatus("Finished", "");
        if ($("#output").children.length === 0) {
            appendLine("(no output)", "meta");
        }
    } catch (e) {
        appendLine(`${e}`, "err");
        setStatus("Error", "error");
    }
};

const handleTab = (e: KeyboardEvent) => {
    if (e.key !== "Tab") return;
    e.preventDefault();
    const ta = e.target as HTMLTextAreaElement;
    const start = ta.selectionStart;
    const end = ta.selectionEnd;
    ta.value = ta.value.substring(0, start) + "  " + ta.value.substring(end);
    ta.selectionStart = ta.selectionEnd = start + 2;
    renderHighlight();
};

const updateCursorPos = () => {
    const ta = $("#code") as HTMLTextAreaElement;
    const upToCursor = ta.value.substring(0, ta.selectionStart);
    const lines = upToCursor.split("\n");
    const line = lines.length;
    const col = lines[lines.length - 1].length + 1;
    $("#cursor-pos").text(`Ln ${line}, Col ${col}`);
};

const setupResizer = () => {
    const gutter = $("#gutter");
    const left = $("#pane-code");
    const body = $("#editor-body");
    if (!gutter || !left || !body) return;

    let dragging = false;
    const isVertical = () => window.innerWidth <= 800;

    gutter.on("mousedown", () => {
        dragging = true;
        gutter.addClass("dragging");
        document.body.css("userSelect", "none");
    });

    window.addEventListener("mousemove", (e: MouseEvent) => {
        if (!dragging) return;
        const rect = body.getBoundingClientRect();
        const ratio = isVertical()
            ? ((e.clientY - rect.top) / rect.height) * 100
            : ((e.clientX - rect.left) / rect.width) * 100;
        left.css("flex", `0 0 ${Math.min(85, Math.max(15, ratio))}%`);
    });

    window.addEventListener("mouseup", () => {
        if (!dragging) return;
        dragging = false;
        gutter.rmClass("dragging");
        document.body.css("userSelect", "auto");
    });
};

const buildEditor = () => {
    const ta = $("#code") as HTMLTextAreaElement;

    $("#run").on("click", runCode);
    $("#clear").on("click", () => {
        clearOutput();
        setStatus("Ready");
    });

    ta.on("keydown", (e: Event) => {
        const ke = e as KeyboardEvent;
        handleTab(ke);
        if (ke.key === "Enter" && (ke.ctrlKey || ke.metaKey)) {
            ke.preventDefault();
            runCode();
        }
    });

    ta.on("keyup", updateCursorPos);
    ta.on("click", updateCursorPos);
    ta.on("input", renderHighlight);
    ta.on("scroll", syncScroll);

    setupResizer();
    updateCursorPos();
    renderHighlight();
};

(window as any).buildEditor = buildEditor;

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", buildEditor);
} else {
    buildEditor();
}
