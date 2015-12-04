// Day 1
// http://adventofcode.com/day/1

function part1(){
	var input = document.getElementsByTagName("pre")[0].innerHTML;
	var floor = 0;

	for(var i = 0; i < input.length; i++){
		input.charAt(i) == "(" ? floor++ : floor--;
	}
	return floor;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].innerHTML;
	var floor = 0;

	for(var i = 0; i < input.length; i++){
		input.charAt(i) == "(" ? floor++ : floor--;
		if(floor < 0){
			return (i + 1);
		}
	}
}
