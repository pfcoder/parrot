/**
 * Created by ly on 8/24/16.
 */

var frameWidth, frameHeight;

ratio = 1334/750;

function main(){
    resizeFrame();
}

function resizeFrame(){
    frameWidth = $(window).width();
    frameHeight = frameWidth*ratio;
    
    var joFrame = $("#frame");
    joFrame.width(frameWidth);
    joFrame.height(frameHeight);
}

$(document).ready(main);
$(window).resize(resizeFrame);


//# sourceMappingURL=master.js.map
