import "htmx.org";

let isNavBarHidden = false;


$("#toggle-code").on("click", (_) => {
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

    if (window.innerWidth < 1200) {
        switchHidden();
    }
}

const setHlPage = (s: string) => {
    $("li").forEach(e => s !== e.getAttribute("hx-replace-url")?.split("/")?.at(-1) ? e.rmClass("active") : e.addClass("active"));
}

const switchHidden = () => {
    if (isNavBarHidden) {
        $("#sidebar").rmClass("hidden");
    } else {
        $("#sidebar").addClass("hidden");
    }

    isNavBarHidden = !isNavBarHidden;
}

$("#hide-sidebar").on("click", switchHidden)


// $("#hidden-code").on("click", (_) => {
//     console.log("wtf");
//     console.log($("#hidden-code"))
//     console.log($("#hidden-code").classList)
//     document.getElementById("hidden-code")?.classList.remove("visible")
//     $("#hidden-code").rmClass("visible");
// })
