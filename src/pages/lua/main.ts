import "htmx.org";
import "hyperscript.org";
import { buildRunners } from "./runner";

const onNewPage = () => {
    buildRunners();
    toggleTechnical();
    hlCurrentPage();
}

const hlCurrentPage = () => {
    const currentUrl = window.location.pathname;
    const currentPage = currentUrl.split("/")[2] ?? "introduction";

    $("li").forEach(e => currentPage !== e.getAttribute("hx-replace-url")?.split("/")?.at(-1) ? e.rmClass("active") : e.addClass("active"));
}


const toggleTechnical = () => {
    $(".for-technical").forEach(el => {
        let currentHtml = el.innerHTML;
        el.innerHTML = "";
        const button = $new("button", {}, "For technical people");
        const content = $new("p", {}, "");
        content.innerHTML = currentHtml;
        button.on("click", () => {
            if (content.hasClass("visible")) {
                content.rmClass("visible");
            } else {
                content.addClass("visible");
            }
        });

        el.add([
            button,
            content
        ])
    })
}

