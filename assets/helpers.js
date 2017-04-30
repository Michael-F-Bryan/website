function editTime(timeID) {
    window.location.href = `/times/${timeID}/edit`;
}

function deleteTime(timeID) {
    $("#yesBtn").attr("href", `/times/${timeID}/delete`);
    $("#areYouSure").modal();
}

