// Register a jquery hook which is run when the page loads
// This should:
//
// - Get the current URL path and set the `active` class on the correct 
//   nav link (if any)

$(document).ready(function() {
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
})