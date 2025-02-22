import "htmx.org";
import { buildRunners } from "./runner";

let isNavBarHidden = false;


$("#toggle-code")?.on("click", (_) => {
    console.log("you?")
    let hiddenCode = $("#hidden-code");
    if (hiddenCode.hasClass("visible")) {
        hiddenCode.rmClass("visible")
    } else {
        hiddenCode.addClass("visible")
    }
})

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
    buildRunners();
    window.scrollTo(0, 0);
}


if (window.innerWidth < 1200) {
    switchHidden();
}

// We want to highlight the current page we're on in the `li`s
hlCurrentPage();
$("#hide-sidebar").on("click", switchHidden);
