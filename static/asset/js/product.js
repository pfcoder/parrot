

var topBar = $("#topbar");
var dropDown = $("#dropDown");
var dropDownItems = $("#dropDown .dropDown-item");

$(document).ready(function(){
    // topBar.click(function(){
    //     dropDown.slideDown();
    //     // dropDown.css('display', "block");
    // });
    // dropDownItems.click(function(){
    //     dropDown.css('display', "none");
    // });
    //
    // $(document).click(function(){
    //     dropDown.css('display', "none");
    // });

    $("#form_div .params_btn").click(function(){
        $("#close_img").css('display', "block");
        $("#curtain").css('display', "block");
        $(".form_img_div:first").css('display', "block");
    });

    $("#close_img").click(function(){
        $("#close_img").css('display', "none");
        $("#curtain").css('display', "none");
        $(".form_img_div:first").css('display', "none");
    });
    
    layoutForm();
    
    var winWidth  = $(window).width();
    var videoWidth = winWidth*0.8;
    var videoRatio = 0.75;
    
    
    $("#video_div .video_iframe").attr("height", videoRatio*videoWidth);
    $("#video_div .video_iframe").attr("width", videoWidth);
    


    
    
});
$("#img_div .prod_img").load(function(){
     /**
     * Layout
     */
    layoutForm();

    
    

});

function layoutForm(){
    var totalHeight = $("#img_div img:first").height()+$("#topbar").height()+($("#form_div").length>0?50:0);
    // alert($("#img_div img:first").height()+"|"+$("#topbar").height());
    // alert(totalHeight);
    $("#curtain").height(totalHeight);
    $("#frame").height(totalHeight);
    
    var scrollTop = $(window).scrollTop();
    var fTop = $(window).height()/2 - $(".form_img_div").height()/2-20;
    $(".form_img_div:first").css("top", fTop+scrollTop+"px");
    $("#close_img").css("top", scrollTop+10+"px");   
    
}

$(window).scroll(function(){
    var scrollTop = $(window).scrollTop();
    var fTop = $(window).height()/2 - $(".form_img_div").height()/2 - 20;
    if($("#curtain").css('display')=="none"){
    $(".form_img_div:first").css("top", fTop+scrollTop+"px");
    }
    $("#close_img").css("top", scrollTop+10+"px");
});
$(window).resize(function(){
    var totalHeight = $("#img_div img:first").height()+$("#topbar").height()+($("#form_div").length>0?50:0);
    $("#curtain").height(totalHeight);

    layoutForm();

    // var scrollTop = $(window).scrollTop();
    // var fTop = $(window).height()/2 - $(".form_img_div").height()/2-20;
    // $(".form_img_div:first").css("top", fTop+scrollTop+"px");
    // $("#close_img").css("top", scrollTop+10+"px");
    



});

//# sourceMappingURL=product.js.map
