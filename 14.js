// Day 14
// http://adventofcode.com/day/14
var totalSeconds = 2503;
var units = "km";
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var reindeer = {};
	function reindeerTick(reindeer){
		for(var name in reindeer){
			var r = reindeer[name];
			if(r.stateRemaining == 0){
				r.flying = !r.flying;
				r.stateRemaining = r.flying ? r.canFly : r.mustRest;
			}
			if(r.flying){
				r.totalDistance += r.speed;
			}
			r.stateRemaining--;
		}
	}

	for(var i in input){
		if(input[i].trim() == "")
			continue;
		var line = input[i].split(" ");
		reindeer[line[0]] = {
			"speed": parseInt(line[3]),
			"canFly": parseInt(line[6]),
			"mustRest": parseInt(line[13]),
			"flying": false,
			"stateRemaining": 0,
			"totalDistance": 0
		};
	}

	for(var s = 0; s < totalSeconds; s++){
		reindeerTick(reindeer);
	}

	var winners = [];
	for(var name in reindeer){
		// console.log(name + ": " + reindeer[name].totalDistance + " " + units);
		if(winners.length == 0 || reindeer[name].totalDistance > reindeer[winners[0]].totalDistance)
			winners = [name];
		else if(reindeer[name].totalDistance == reindeer[winners[0]].totalDistance)
			winners.push(name);
	}
	if(winners.length > 1)
		console.log("There was a tie!");
	for(var i in winners){
		console.log(winners[i] + " wins with a total distance of " + reindeer[winners[i]].totalDistance + " " + units);
	}
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var reindeer = {};
	function reindeerTick(reindeer){
		var inFirst = [];
		for(var name in reindeer){
			var r = reindeer[name];
			if(r.stateRemaining == 0){
				r.flying = !r.flying;
				r.stateRemaining = r.flying ? r.canFly : r.mustRest;
			}
			if(r.flying){
				r.totalDistance += r.speed;
			}
			r.stateRemaining--;
			if(inFirst.length == 0 || r.totalDistance > reindeer[inFirst[0]].totalDistance)
				inFirst = [name];
			else if(r.totalDistance == reindeer[inFirst[0]].totalDistance)
				inFirst.push(name);
		}
		inFirst.forEach(function(name){
			reindeer[name].points++;
		});
	}

	for(var i in input){
		if(input[i].trim() == "")
			continue;
		var line = input[i].split(" ");
		reindeer[line[0]] = {
			"speed": parseInt(line[3]),
			"canFly": parseInt(line[6]),
			"mustRest": parseInt(line[13]),
			"flying": false,
			"stateRemaining": 0,
			"totalDistance": 0,
			"points": 0
		};
	}

	for(var s = 0; s < totalSeconds; s++){
		reindeerTick(reindeer);
	}

	console.log(reindeer);

	var winners = [];
	for(var name in reindeer){
		if(winners.length == 0 || reindeer[name].points > reindeer[winners[0]].points)
			winners = [name];
		else if(reindeer[name].points == reindeer[winners[0]].points)
			winners.push(name);
	}
	if(winners.length > 1)
		console.log("There was a tie!");
	for(var i in winners){
		console.log(winners[i] + " wins with a total of " + reindeer[winners[i]].points + " points");
	}
}
