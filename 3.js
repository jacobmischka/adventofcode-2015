// Day 3
// http://adventofcode.com/day/3

function part1(){
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
	return houses;
}

function part2(){
	var grid = [[]];
	grid[0][0] = 1;
	var input = document.getElementsByTagName("pre")[0].textContent;
	var human = {x: 0, y: 0}, robot = {x: 0, y: 0}, houses = 1, gifts = 1;
	var santa;
	for(var i = 0; i < input.length; i++){
		direction = input.charAt(i);
		if(i%2)
			santa = human;
		else
			santa = robot;
		switch(direction){
			case "^":
				santa.y++;
				break;
			case "v":
				santa.y--;
				break;
			case ">":
				santa.x++;
				break;
			case "<":
				santa.x--;
				break;
		}
		if(typeof(grid[santa.x]) == "undefined")
			grid[santa.x] = [];
		if(typeof(grid[santa.x][santa.y]) == "undefined"){
			houses++;
			grid[santa.x][santa.y] = 0;
		}
		grid[santa.x][santa.y]++;
		gifts++;
	}
	return houses;
}
