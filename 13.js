// Day 13
// http://adventofcode.com/day/13
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var happinesses = {};
	for(var i in input){
		if(input[i].trim() == "")
			continue;
		var line = input[i].substring(0, input[i].length-1).split(" ");
		if(typeof(happinesses[line[0]]) == "undefined")
			happinesses[line[0]] = {};
		happinesses[line[0]][line[line.length-1]] = parseInt(line[3]) * (line[2] == "gain" ? 1 : -1);
	}

	var people = Object.keys(happinesses);
	happinesses.get = function(a, b){
		return this[a][b] + this[b][a];
	};
	var arrangements = permutation(people);
	var arrangement = [], bestHappiness = -Infinity;
	for(var i in arrangements){
		var happiness = happinesses.get(arrangements[i][0], arrangements[i][arrangements[i].length-1]);
		for(var j = 0; j < arrangements[i].length-1; j++){
			happiness += happinesses.get(arrangements[i][j], arrangements[i][j+1]);
		}
		if(happiness > bestHappiness){
			bestHappiness = happiness;
			arrangement = arrangements[i];
		}
	}
	return bestHappiness;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var happinesses = {"Me":{}};
	for(var i in input){
		if(input[i].trim() == "")
			continue;
		var line = input[i].substring(0, input[i].length-1).split(" ");
		if(typeof(happinesses[line[0]]) == "undefined")
			happinesses[line[0]] = {};
		happinesses[line[0]][line[line.length-1]] = parseInt(line[3]) * (line[2] == "gain" ? 1 : -1);
		happinesses[line[0]]["Me"] = 0;
		happinesses["Me"][line[0]] = 0;
	}

	var people = Object.keys(happinesses);
	happinesses.get = function(a, b){
		return this[a][b] + this[b][a];
	};
	var arrangements = permutation(people);
	var arrangement = [], bestHappiness = -Infinity;
	for(var i in arrangements){
		var happiness = happinesses.get(arrangements[i][0], arrangements[i][arrangements[i].length-1]);
		for(var j = 0; j < arrangements[i].length-1; j++){
			happiness += happinesses.get(arrangements[i][j], arrangements[i][j+1]);
		}
		if(happiness > bestHappiness){
			bestHappiness = happiness;
			arrangement = arrangements[i];
		}
	}
	return bestHappiness;
}

function permutation(arr){
	var permutations = [];

	function permute(a, saved){
		var place, saved = saved || [];
		for(var i = 0; i < a.length; i++){
			place = a.splice(i, 1);
			if(a.length === 0)
				permutations.push(saved.concat(place));
			permute(a.slice(), saved.concat(place));
			a.splice(i, 0, place[0]);
		}

		return permutations;
	}

	return permute(arr.slice());
};
