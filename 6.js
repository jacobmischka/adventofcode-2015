// Day 6
// http://adventofcode.com/day/6

function part1(){
	var input = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var lights = [[]];
	var lightsOn = 0;
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() == "")
			continue;
		var command = input[i].replace("turn ", "turn");
		command = command.split(" ");
		var start = command[1].split(",");
		var end = command[3].split(",");
		for(var x = parseInt(start[0]); x <= parseInt(end[0]); x++){
			if(typeof(lights[x]) == "undefined")
				lights[x] = [];
			for(var y = parseInt(start[1]); y <= parseInt(end[1]); y++){
				if(typeof(lights[x][y]) == "undefined")
					lights[x][y] = false;
				switch(command[0]){
					case "turnon":
						if(!lights[x][y])
							lightsOn++;
						lights[x][y] = true;
						break;
					case "turnoff":
						if(lights[x][y])
							lightsOn--;
						lights[x][y] = false;
						break;
					case "toggle":
						lights[x][y] ? lightsOn-- : lightsOn++;
						lights[x][y] = !lights[x][y];
						break;
				}
			}
		}
	}
	return lightsOn;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var lights = [[]];
	var brightness = 0;
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() == "")
			continue;
		var command = input[i].replace("turn ", "turn");
		command = command.split(" ");
		var start = command[1].split(",");
		var end = command[3].split(",");
		for(var x = parseInt(start[0]); x <= parseInt(end[0]); x++){
			if(typeof(lights[x]) == "undefined")
				lights[x] = [];
			for(var y = parseInt(start[1]); y <= parseInt(end[1]); y++){
				if(typeof(lights[x][y]) == "undefined")
					lights[x][y] = 0;
				switch(command[0]){
					case "turnon":
						brightness++;
						lights[x][y]++;
						break;
					case "turnoff":
						if(lights[x][y] > 0){
							brightness--;
							lights[x][y]--;
						}
						break;
					case "toggle":
						brightness += 2;
						lights[x][y] += 2;
						break;
				}
			}
		}
	}
	return brightness;
}
