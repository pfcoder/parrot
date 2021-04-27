function clearActive(objs, className) {
    objs.each(function () {
        if ($(this).hasClass(className)) {
            $(this).removeClass(className);
        }
    })
}
function setActive(activeTab, oTabs, activeClassName){
    clearActive(oTabs, activeClassName);
    activeTab.addClass(activeClassName);

    $("#down_arrow .arrow_img").css("left", activeTab.position().left+"px");
    displayContent(activeTab);
    
}

function displayContent(activeTab) {
    var order = activeTab.attr("order");
    var oContents = $(".products");
    oContents.each(function(i){
        if(i!=order){
            $("#content_"+i).css("display","none");
        }else{
            $("#content_"+i).css("display","block");
        }
    });
}


$(document).ready(function(){
    
    var oTabs = $("#topbar .tab");

    if(activeSeriesId){
        var at = oTabs.eq(activeSeriesId)
        setActive(at, oTabs, 'tab_active');
    }

    oTabs.click(function(){
        setActive($(this), oTabs, "tab_active");
    });
    
});

$(document).resize(function(){
});



//# sourceMappingURL=series.js.map
