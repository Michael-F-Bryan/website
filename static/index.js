$(document).ready(function() {
    setActiveNavItem()
    activateTooltips()
    entryEditFormValidation()
})

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

function entryEditFormValidation() {
    $("#entry-edit").submit(e => {
        const form = $("#entry-edit")
        var isValid = true

        const start = $("#start").val()
        const end = $("#end").val()

        if (!startIsBeforeEnd(start, end)) {
            $("#start-after-end").show().delay(4000).fadeOut(1000)
            isValid = false
        }

        if (!isValid) {
            e.preventDefault()
        }
    })
}

/**
 * Interpret the provided strings as times and ensure that start is before end.
 * 
 * @param {string} start - The start time, a string formatted like "17:00"
 * @param {string} end - The end time, a string formatted like "17:00"
 */
function startIsBeforeEnd(start, end) {
    start = start.split(":").map(i => parseInt(i))
    end = end.split(":").map(i => parseInt(i))
    console.log(start, end)

    const startTime = start[0] * 60 + start[1]
    const endTime = end[0] * 60 + end[1]
    console.log(startTime, endTime)
    return startTime <= endTime
}