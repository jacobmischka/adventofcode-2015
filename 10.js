// Day 10
// http://adventofcode.com/day/10

function part1(){
	var input = "1113222113";
	for(var i = 0; i < 40; i++){
		var digits = [{digit: input.charAt(0), count: 1}];
		for(var j = 1; j < input.length; j++){
			if(input.charAt(j) == digits[digits.length - 1].digit)
				digits[digits.length-1].count++;
			else
				digits.push({digit: input.charAt(j), count: 1});
		}
		input = "";
		for(var seq of digits){
			input += seq.count + seq.digit;
		}
	}
	return input.length;
}

function part2(){
	var input = "1113222113";
	for(var i = 0; i < 50; i++){
		var digits = [{digit: input.charAt(0), count: 1}];
		for(var j = 1; j < input.length; j++){
			if(input.charAt(j) == digits[digits.length - 1].digit)
				digits[digits.length-1].count++;
			else
				digits.push({digit: input.charAt(j), count: 1});
		}
		input = "";
		for(var seq of digits){
			input += seq.count + seq.digit;
		}
	}
	return input.length;
}
