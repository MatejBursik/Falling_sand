let canvX = 400;
let canvY = 400;
let scale = 5;

stepUp(canvX, canvY);

let grid = fillZeros(make2darr(canvX/scale, canvY/scale));
let clicked = false;
let mouseY, mouseX;
const canv = document.getElementById("container");

let spawn = 0;
let color = 0;

function game(scale){
    draw(grid,scale);
    grid = fall(grid);

    if (spawn > 5){
        if (color > 359){
            color = 0;
        }

        grid[0][canvX/scale/10] = color;
        grid[0][canvX/scale/2] = color;
        
        spawn = 0;
    }

    if (clicked){
        grid[parseInt(mouseY/scale)][parseInt(mouseX/scale)] = color;
    }
    
    spawn++;
    color++;
}

document.onmousemove = function(event){
    mouseY = event.pageY;
    mouseX= event.pageX;
}

canv.addEventListener("mousedown", (event) => {
    clicked = true;
});

canv.addEventListener("mouseup", (event) => {
    clicked = false;
});

setInterval(game, 5, scale);
