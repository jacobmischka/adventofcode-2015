// http://adventofcode.com/day/3
// Part 1
var grid = [[]];
grid[0][0] = 1;
var input = document.getElementsByTagName("pre")[0].textContent;
var x = 0, y = 0, houses = 1, gifts = 1;
for(var i = 0; i < input.length; i++){
	direction = input.charAt(i);
	switch(direction){
		case "^":
			y++;
			break;
		case "v":
			y--;
			break;
		case ">":
			x++;
			break;
		case "<":
			x--;
			break;
	}
	if(typeof(grid[x]) == "undefined")
		grid[x] = [];
	if(typeof(grid[x][y]) == "undefined"){
		houses++;
		grid[x][y] = 0;
	}
	grid[x][y]++;
	gifts++;
}

// Part 2
var grid = [[]];
grid[0][0] = 1;
var input = document.getElementsByTagName("pre")[0].textContent;
var santaX = 0, santaY = 0, robosantaX = 0, robosantaY = 0, houses = 1, gifts = 1;
for(var i = 0; i < input.length; i++){
	direction = input.charAt(i);
	if(i%2){
		x = santaX;
		y = santaY;
	}
	else{
		x = robosantaX;
		y = robosantaY;
	}
	switch(direction){
		case "^":
			y++;
			break;
		case "v":
			y--;
			break;
		case ">":
			x++;
			break;
		case "<":
			x--;
			break;
	}
	if(typeof(grid[x]) == "undefined")
		grid[x] = [];
	if(typeof(grid[x][y]) == "undefined"){
		houses++;
		grid[x][y] = 0;
	}
	grid[x][y]++;
	gifts++;
	if(i%2){
		santaX = x;
		santaY = y;
	}
	else{
		robosantaX = x;
		robosantaY = y;
	}
}
