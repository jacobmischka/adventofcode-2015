// Day 15
// http://adventofcode.com/day/15
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	for(var i in input){
		if(input[i].trim() == ""){
			input.splice(i, 1);
			continue;
		}
		var line = input[i].split(" ");
		for(var j in line){
			if(j % 2)
				line[j] += ":";
		}
		input[i] = line.join(" ").replace(":", ":{") + "},";
	}
	var ingredients = eval("({" + input.join("\n") + "})");
	// return ingredients;
	var partitions = getPartitions(100);
	// console.log(combinations);
	var maxScore = 0;
	for(var i in partitions){
		// console.log(permutations);
		var capacity = 0, durability = 0, flavor = 0, texture = 0;
		for(var j in ingredients){
			// console.log(ingredients[k]);
			// console.log(permutations[j]);
			// console.log(Object.keys(ingredients).indexOf(k));
			capacity += ingredients[j].capacity * partitions[i][Object.keys(ingredients).indexOf(j)];
			durability += ingredients[j].durability * partitions[i][Object.keys(ingredients).indexOf(j)];
			flavor += ingredients[j].flavor * partitions[i][Object.keys(ingredients).indexOf(j)];
			texture += ingredients[j].texture * partitions[i][Object.keys(ingredients).indexOf(j)];
		}
		if(capacity < 0 || durability < 0 || flavor < 0 || texture < 0)
			score = 0;
		else
			score = capacity * durability * flavor * texture;
		if(score > maxScore)
			maxScore = score;
	}
	return maxScore;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	for(var i in input){
		if(input[i].trim() == ""){
			input.splice(i, 1);
			continue;
		}
		var line = input[i].split(" ");
		for(var j in line){
			if(j % 2)
				line[j] += ":";
		}
		input[i] = line.join(" ").replace(":", ":{") + "},";
	}
	var ingredients = eval("({" + input.join("\n") + "})");
	// return ingredients;
	var partitions = getPartitions(100);
	// console.log(combinations);
	var maxScore = 0;
	for(var i in partitions){
		// console.log(permutations);
		var capacity = 0, durability = 0, flavor = 0, texture = 0, calories = 0;
		for(var j in ingredients){
			// console.log(ingredients[k]);
			// console.log(permutations[j]);
			// console.log(Object.keys(ingredients).indexOf(k));
			capacity += ingredients[j].capacity * partitions[i][Object.keys(ingredients).indexOf(j)];
			durability += ingredients[j].durability * partitions[i][Object.keys(ingredients).indexOf(j)];
			flavor += ingredients[j].flavor * partitions[i][Object.keys(ingredients).indexOf(j)];
			texture += ingredients[j].texture * partitions[i][Object.keys(ingredients).indexOf(j)];
			calories += ingredients[j].calories * partitions[i][Object.keys(ingredients).indexOf(j)];
		}
		if(capacity < 0 || durability < 0 || flavor < 0 || texture < 0)
			score = 0;
		else
			score = capacity * durability * flavor * texture;
		if(score > maxScore && calories == 500)
			maxScore = score;
	}
	return maxScore;
}

function getPartitions(n){
	var partitions = [];
	for(var i = 0; i < n; i++){
		for(var j = 0; j < n; j++){
			for(var k = 0; k < n; k++){
				var l = n - (i + j + k);
				partitions.push([i, j, k, l]);
			}
		}
	}
	return partitions;
}
