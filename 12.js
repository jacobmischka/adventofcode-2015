// Day 12
// http://adventofcode.com/day/12
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.replace(/[^\d\-]+/g, "+");
	if(input[input.length-1] == "+")
		input = input.substring(0, input.length-1);
	return eval(input);
}

function part2(){
	var input = JSON.parse(document.getElementsByTagName("pre")[0].textContent);
	
	function addChildren(obj){
		if(typeof(obj) == "object"){
			var sum = 0;
			for(var i in obj){
				if(typeof(obj[i]) == "object")
					sum += addChildren(obj[i]);
				else if(obj[i] == "red" && obj.constructor !== Array)
					return 0;
				else if(!isNaN(obj[i]))
					sum += parseFloat(obj[i]);
			}
			return sum;
		}
		if(!isNaN(obj))
			return obj;
		return 0;
	}

	return addChildren(input);
}
