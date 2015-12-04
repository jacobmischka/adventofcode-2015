// Day 2
// http://adventofcode.com/day/2

function part1(){
	var lines = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var total = 0;
	for(var i = 0; i < lines.length; i++){
		if(lines[i].trim() == "")
			continue;
		var x = lines[i].split("x");
		var s = [x[0]*x[1], x[1]*x[2], x[2]*x[0]];
		var min = -1;
		s.forEach(function(a){
			if(a < min || min == -1)
				min = a;
		});
		total += (2 * parseInt(s[0]+s[1]+s[2]) + parseInt(min));
	}
	return total;
}

function part2(){
	var lines = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var total = 0;
	for(var i = 0; i < lines.length; i++){
		if(lines[i].trim() == "")
			continue;
		var x = lines[i].split("x");
		x.sort(function(a, b){
			return a - b;
		});
		total += (2 * (parseInt(x[0]) + parseInt(x[1]))) + (x[0] * x[1] * x[2])
	}
	return total;
}
