import "htmx.org";
import { toggleClass } from "./toggle";
import { buildRunners } from "./runner";

let isNavBarHidden = false;


const hlCurrentPage = () => {
    const currentUrl = window.location.pathname;
    const currentPage = currentUrl.split("/")[2] ?? "introduction";

    setHlPage(currentPage);

    if (window.innerWidth < 1200 && !isNavBarHidden) {
        switchHidden();
    }
}

const setHlPage = (s: string) => {
    // $("#hide-sidebar").removeEventListener("click", switchHidden);
    $("li").forEach(e => s !== e.getAttribute("hx-replace-url")?.split("/")?.at(-1) ? e.rmClass("active") : e.addClass("active"));
}

const switchHidden = () => {
    if (isNavBarHidden) {
        $("#sidebar").rmClass("hidden");
        $("#hide-sidebar").text("<");
    } else {
        $("#sidebar").addClass("hidden");
        $("#hide-sidebar").text(">");
    }

    isNavBarHidden = !isNavBarHidden;
}

const onClickNewPage = (newPage: string) => {
    console.log("pls");
    setHlPage(newPage)
    recCheckNewPage($("#content").classList[0], 0);

}

const recCheckNewPage = (notClass: string, tries: number) => {
    setTimeout(() => {
        console.log("uwu")
        console.log($("#content").classList, notClass)
        if (!$("#content").classList.contains(notClass)) {
            onNewPage()
        } else if (tries < 1000) {
            recCheckNewPage(notClass, tries + 1)
        }
    }, 10);
}

const onNewPage = () => {
    hlCurrentPage();
    toggleTechnical();
    buildRunners();
    $('main').forEach(main => main.scrollTop = 0);
    setTimeout(() => {
        window.scrollTo(0,1);
    }, 10);
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

toggleTechnical();

if (window.innerWidth < 1200) {
    switchHidden();
}

// We want to highlight the current page we're on in the `li`s
hlCurrentPage();
$("#hide-sidebar").on("click", switchHidden);
