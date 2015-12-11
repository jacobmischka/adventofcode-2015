// Day 9
// http://adventofcode.com/day/9
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var distances = {};
	var places = [];
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() != ""){
			var line = input[i].trim().split(" ");
			if(typeof(distances[line[0]]) == "undefined")
				distances[line[0]] = {};
			if(typeof(distances[line[2]]) == "undefined")
				distances[line[2]] = {};
			distances[line[0]][line[2]] = parseInt(line[4]);
			distances[line[2]][line[0]] = parseInt(line[4]);
		}
	}
	places = Object.keys(distances);
	places.permutations = function(){
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

		return permute(this.slice());
	};

	var shortestDistance = Infinity;

	for(var permutation of places.permutations()){
		permutation.distance = 0;
		for(var i = 1; i < permutation.length; i++){
			if(typeof(permutation[i-1]) == "undefined")
				console.log(permutation);
			permutation.distance += distances[permutation[i-1]][permutation[i]];
		}
		if(permutation.distance < shortestDistance)
			shortestDistance = permutation.distance;
	}

	return shortestDistance;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var distances = {};
	var places = [];
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() != ""){
			var line = input[i].trim().split(" ");
			if(typeof(distances[line[0]]) == "undefined")
				distances[line[0]] = {};
			if(typeof(distances[line[2]]) == "undefined")
				distances[line[2]] = {};
			distances[line[0]][line[2]] = parseInt(line[4]);
			distances[line[2]][line[0]] = parseInt(line[4]);
		}
	}
	places = Object.keys(distances);
	places.permutations = function(){
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

		return permute(this.slice());
	};

	var longestDistance = 0;

	for(var permutation of places.permutations()){
		permutation.distance = 0;
		for(var i = 1; i < permutation.length; i++){
			if(typeof(permutation[i-1]) == "undefined")
				console.log(permutation);
			permutation.distance += distances[permutation[i-1]][permutation[i]];
		}
		if(permutation.distance > longestDistance)
			longestDistance = permutation.distance;
	}

	return longestDistance;
}
