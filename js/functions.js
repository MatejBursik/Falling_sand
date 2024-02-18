function make2darr(x,y){
    let arr = new Array(y);

    for (let i=0; i<arr.length; i++){
        arr[i] = new Array(x);
    }

    return arr;
}

function fillZeros(arr){
    for (let i=0; i<arr.length; i++){
        for (let j=0; j<arr[i].length; j++){
            arr[i][j] = 0;
        }
    }

    return arr;
}

Array.prototype.choose = function(){
    return this[Math.floor(Math.random()*this.length)];
}

function stepUp(width, height){
    const canvas = document.getElementById("container");
    canvas.width = width;
    canvas.height = height;
}

function draw(arr, scale){
    const canvas = document.getElementById("container");
    const ctx = canvas.getContext("2d");
    
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    for (let y=0; y<arr.length; y++){
        for (let x=0; x<arr[y].length; x++){
            if (arr[y][x] > 0){
                // hsl 0 - 359
                ctx.fillStyle = 'hsl(' + arr[y][x] + ', 100%, 50%)';
                ctx.fillRect(x*scale, y*scale, scale, scale);
            }
        }
    }
}

function fall(arr){
    let newArr = fillZeros(make2darr(arr[0].length, arr.length));
    let dir = [1,-1].choose();

    for (let y=0; y<arr.length; y++){
        for (let x=0; x<arr[y].length; x++){
            if (arr[y][x] > 0){
                if (y+1 < arr.length){
                    if (arr[y+1][x] === 0){
                        newArr[y+1][x] = arr[y][x];
                    } else if (x+1 < arr[0].length && dir === 1){
                        if (arr[y+1][x+1] === 0){
                            newArr[y+1][x+1] = arr[y][x];
                        } else {
                            newArr[y][x] = arr[y][x];
                        }
                    } else if (x-1 >= 0 && dir === -1){
                        if (arr[y+1][x-1] === 0){
                            newArr[y+1][x-1] = arr[y][x];
                        } else {
                            newArr[y][x] = arr[y][x];
                        }
                    } else {
                        newArr[y][x] = arr[y][x];
                    }
                } else {
                    newArr[y][x] = arr[y][x];
                }
            }
        }
    }

    return newArr;
}
