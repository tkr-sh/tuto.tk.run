const toggleClass = (id: string, classToToggle: string) => {
    const el = $("#" + id);
    if (el.hasClass(classToToggle)) {
        el.rmClass(classToToggle);
    } else {
        el.addClass(classToToggle);
    }

}
