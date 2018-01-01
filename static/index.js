
/**
 * Goes through the links in the navbar and applies the `active` to the one
 * we're currently on.
 */
function setActiveNavItem() {
    let items = $("#nav-list .nav-item")
        .filter((i, item) => {
            return $(item).find(".nav-link")
        })

    for (var item of items) {
        item = $(item)
        const href = item.find("a").attr("href")

        if (href == document.location.pathname) {
            item.addClass("active")
            return
        }
    }
}

/**
 * Activates any tooltips which may be on the page.
 */
function activateTooltips() {
    $('[data-toggle="tooltip"]').tooltip()
}

$(document).ready(function() {
    setActiveNavItem()
    activateTooltips()
})