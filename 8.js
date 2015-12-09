// Day 8
// http://adventofcode.com/day/8

function part1(){
	var diff = 0;
	var input = document.getElementsByTagName("pre")[0].innerText.split("\n");
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() != "")
			diff += input[i].length - eval(input[i]).length;
	}
	return diff;
}

function part2(){
	var diff = 0;
	var input = document.getElementsByTagName("pre")[0].innerText.split("\n");
	for(var i = 0; i < input.length; i++){
		if(input[i].trim() != ""){
			var withslashes = '"' + input[i].replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"';
			diff += withslashes.length - input[i].length;
		}
	}
	return diff;
}
