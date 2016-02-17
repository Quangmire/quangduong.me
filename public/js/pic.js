console.log("c");
$(document).ready(function() {
    $(".pic").hover(function(e) {
        var content = $(this).find("a").attr("href");
        var w = $(this).attr("w");
        var h = $(this).attr("h");
        var l = parseInt($(this).width()) - 10;
        var t = parseInt(h) + 10;
        var img = "<img class=\"hover-pic\" width=\"" + w + "\" height=\"" + h + "\"style=\"margin-left:-" + l + "px;margin-top:-" + t + "px;\" src=\"" + content + "\"/>";
        if(e.type === "mouseenter") {
            $(this).append(img);
        } else {
            $(this).find(".hover-pic").remove();
        }
    });
});
