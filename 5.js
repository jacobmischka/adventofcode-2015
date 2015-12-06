// Day 5
// http://adventofcode.com/day/5

function part1(){
	var input = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var vowels = ["a", "e", "i", "o", "u"];
	var blacklist = ["ab", "cd", "pq", "xy"];
	var niceStrings = 0;
	for(var i = 0; i < input.length; i++){
		var s = input[i];
		var vowelcount = 0;
		var doubleletter = false;
		var blacklisted = false;
		var previousLetter = "0";
		for(var j = 0; j < s.length; j++){
			var letter = s.charAt(j);
			if(vowels.indexOf(letter) !== -1)
				vowelcount++;
			if(letter == previousLetter)
				doubleletter = true;
			if(blacklist.indexOf(previousLetter + letter) !== -1){
				blacklisted = true;
				break;
			}
			previousLetter = letter;
		}
		if(!blacklisted && doubleletter && vowelcount >= 3)
			niceStrings++;
	}

	return niceStrings;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].innerHTML.split("\n");
	var niceStrings = 0;
	for(var i = 0; i < input.length; i++){
		var s = input[i];
		if(s.trim() == "")
			continue;
		var xy_xy = false;
		var xyx = false;

		if(s.substring(2).indexOf(s.substring(0, 2)) != -1)
			xy_xy = true;

		for(var j = 2; j < s.length; j++){
			var letter = s.charAt(j);
			if(s.substring(j+1).indexOf(s.substring(j-1, j+1)) != -1)
				xy_xy = true;
			if(letter == s.charAt(j-2))
				xyx = true;
		}
		if(xyx && xy_xy)
			niceStrings++;
	}

	return niceStrings;
}
