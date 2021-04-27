var itemWidth, leftMoveBoundary, rightMoveBoundary, childCount;
var position, flingSpeed, onFling;
var items, carousel, drag;
var ratio, carouselRatio=0.40625;
var carousel;

var fieldsHeight;

function removeTrasition() {
    $(items).css("transition", "none");
    $(items).css("-moz-transition", "none");
    $(items).css("-webkit-transition", "none");
    $(items).css("-o-transition", "none");
}
function addTransition(){
    $(items).css("transition", "left 0.5s");
    $(items).css("-moz-transition", "left 0.5s");
    $(items).css("-webkit-transition", "left 0.5s");
    $(items).css("-o-transition", "left 0.5s");
}
$(document).ready(function(){

    /**
     *  layout indicators
     */
    carousel = $("#carousel");
    var carouselHeight = $(window).width()*carouselRatio;
    carousel.height(carouselHeight);
    
    
    var oIndicators = $("#carousel .position-indicators");
    var initIndicatorWidth = 160;
    var indicatorsWidth = 160*itemNum/5;
    oIndicators.width(indicatorsWidth);
    oIndicators.css('margin-left', -indicatorsWidth/2);


    /**
     * Carousel  Part
     */
    drag = false;
    carousel = document.getElementById("carousel");
    items = document.getElementById("list_items");
    items.style.left = 0;
    // var test = document.getElementById("test");
    // var end = document.getElementById("end");
    var lastX = 0;
    position = 0;
    childCount = $(items).children().length;
    itemWidth = $(window).width();
    leftMoveBoundary = itemWidth*(-1 * (childCount-1));
    rightMoveBoundary = 0;
    
    
    var periods = [];
    var distances = [];
    var totalPeriods = 10;
    var startTime = 0;
    var lastPeriod = 0;
    var lastDistance = 0;
    flingSpeed = itemWidth/300;
    var flingThreshold = 0.3;
    var moveThreshold = 10;
    var timer;
    var curMove = 0;

    onFling = false;
    carousel.addEventListener('touchstart', touch, false);
    carousel.addEventListener('touchmove', touch, false);
    carousel.addEventListener('touchend', touch, false);
    // carousel.addEventListener('ondrag', touch, false);
    carousel.addEventListener('ondragend', touch, false);
    carousel.addEventListener('ondragstart', touch, false);
    carousel.addEventListener('onmousemove', touch, false);



    var timer = setInterval(function(){
        if(position<childCount-1&&position>=0){
            moveToPosition(position+1, flingSpeed);
        }else if(position>=childCount-1) {
            moveToPosition(0, flingSpeed);
        }
    }, 3000);
    
    // test.innerHTML = position;
    
    
    function touch(event){
        if(onFling == false){
            var e = event;
            event.stopPropagation();

            switch(e.type){
                case "touchstart"||"ondragstart":
                    drag = true;
                    // test.innerHTML = "ondragstart";
                    if(onFling){
                        clearInterval(timer);
                        onFling = false;
                    }
                    var date = new Date();
                    startTime = date.getTime();
                    lastX = event.touches[0].pageX;
                    break;
                case "touchmove"||"onmousemove":
                    if(drag==true){
                        // test.innerHTML = "drag";
                        moveItem(event);
                    }
                    break;
                case "touchend"||"ondragend":
                    drag = false;
                    //calculate flingspeed
                    var fs = calculateFlingSpeed();
                    if(Math.abs(fs)>flingThreshold){
                        moveToPosition(fs>0?position-1:position+1, flingSpeed);
                    }else{
                        bounceToNearest();
                    }
                    currentTouchMove = [];
                    break;
            }
        }else{
            
        }
    }
    
    function calculateFlingSpeed(){
        var distance = 0;
        var time = 0;
        for(i=0;i<periods.length;i++){
            distance += distances[i];
            time += periods[i];
        }

        clearMoveCache();
        return distance/time;
    }
    
    function clearMoveCache(){
        //clear for next time
        distances = [];
        periods = [];
        startTime = 0;
        lastPeriod = 0;
    }
    

    
    function bounceToNearest(){
        var curPosition = Math.abs(Math.ceil(parseInt(items.style.left)/itemWidth));
        var rightDist = Math.abs(parseInt(items.style.left)%itemWidth);
        // test.innerHTML = rightDist + "|" + itemWidth;
        if(rightDist<itemWidth/2){
            moveToPosition(curPosition, flingSpeed);
        }else{
            moveToPosition(curPosition+1, flingSpeed);
        }
    }
    
    
    
    

    
    
    

    
    
    //move on touchmove
    function moveItem(event){
        var curX = event.touches[0].pageX;
        var delta =  curX - lastX;
        var dest = parseInt(items.style.left) + delta;
        removeTrasition();

        if(dest<=leftMoveBoundary){
            dest = leftMoveBoundary;
            items.style.left =  dest + "px";
        }
        else if(dest>=rightMoveBoundary){
            dest = rightMoveBoundary;
            items.style.left =  dest + "px";
        }
        else{
            // test.innerHTML = items.style.left + "|" + dest;
            items.style.left =  dest + "px";
            
        }


            
        

        lastX = curX;
        
        //calculate flingspeed
        lastDistance = delta;
        var date = new Date();
        var endTime = date.getTime();
        lastPeriod = endTime - startTime;
        startTime = endTime;

        //add periods to array
        if(distances.length<totalPeriods){
            distances.push(lastDistance);
            periods.push(lastPeriod);
        }
        else{
            distances.shift();
            periods.shift();
            distances.push(lastDistance);
            periods.push(lastPeriod);
        }
    }

    /**
     * fields
     */
    if($(window).width()>$(window).height()){
        ratio = 1334/750;
        fieldsHeight = $(window).width()*ratio;
        $("#fields").height(fieldsHeight);
    }
    else{
        fieldsHeight = $(window).height()-$("#carousel").height();
        $("#fields").height(fieldsHeight);
        ratio = fieldsHeight/$(window).width();
    }
    

});



$(window).resize(function(){


    carousel = $("#carousel");
    var carouselHeight = $(window).width()*carouselRatio;
    carousel.height(carouselHeight);
    
    itemWidth = $(window).width();
    leftMoveBoundary = itemWidth*(-1 * (childCount-1));
    rightMoveBoundary = 0;
    flingSpeed = itemWidth/300;
    // alert(flingSpeed+"|"+ position);
    moveToPosition(position, flingSpeed);

    
    // /**
    //  * fields
    //  */
    // fieldsHeight = ratio*$(window).width();
    // $("#fields").height(fieldsHeight);
    
    if($(window).width()>$(window).height()){
        ratio = 1334/750;
        fieldsHeight = $(window).width()*ratio;
        $("#fields").height(fieldsHeight);
    }
    else{
        fieldsHeight = $(window).height()-$("#carousel").height();
        $("#fields").height(fieldsHeight);
        ratio = fieldsHeight/$(window).width();
    }

});

function setActive(pos){
    var newActiveIndicator = $("#carousel .position-indicators div:nth-child("+(pos+1)+")");
    var selectedIndicator = $("#carousel .position-indicators").find(".indicator-active");
    
    selectedIndicator.removeClass("indicator-active").addClass("indicator");
    newActiveIndicator.removeClass("indicator"). addClass("indicator-active");
}

function moveToPosition(pos, moveSpeed){
    addTransition();
    // test.innerHTML = "resize";
    // test.innerHTML = "cur: " + position + "dest: " + pos;
    if(pos>=0&&pos<=childCount-1){
        var destLeft = -pos*itemWidth;
        var currentLeft = parseInt(items.style.left);
        var distance = destLeft - currentLeft;
        var moveTime = Math.abs(distance/moveSpeed);
        moveContinuously(distance, moveTime, 10);
        // alert(pos);
        position = pos;
        setActive(pos);

    }else{
    }
}
function moveContinuously(distance, time, interval){
    atomicMove(distance);
}

// function moveContinuously(distance, time, interval){
//     var times = Math.floor(time/interval);
//     var step = Math.ceil(distance/(times));
//     // test.innerHTML= "distance: " + distance + "|step: " + step + "|times: " + times;
//     var movedTime = 0;
//     var movedDistance = 0
//     if(distance!=0){
//         onFling = true;
//     }
//     timer =  setInterval(function(){
//         if(Math.abs(movedDistance+step)>Math.abs(distance)){
//             atomicMove(distance - movedDistance);
//         }
//         else{
//             atomicMove(step);
//         }
//        
//         movedTime += 1;
//         movedDistance += step;
//         if(Math.abs(movedDistance)>=Math.abs(distance)){
//             onFling = false;
//             clearInterval(timer);
//         }
//     }, interval);
// }
function atomicMove(step){
    items.style.left =  parseInt(items.style.left) + step + "px";
}

//# sourceMappingURL=pro_field.js.map
