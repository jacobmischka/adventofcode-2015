// Day 16
// http://adventofcode.com/day/16
function part1(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var xhr = new XMLHttpRequest();
	xhr.open("GET", "http://adventofcode.com/day/16", false);
	xhr.send(null);
	var parser = new DOMParser();
	var dayPage = parser.parseFromString(xhr.responseText, "text/html");
	var mfcsamTicker = "({" + dayPage.getElementsByTagName("pre")[0].firstChild.textContent.replace(/\n/g, ", ") + "})";
	var mfcsam = eval(mfcsamTicker);

	for(var i in input){
		if(input[i].trim() == ""){
			input.splice(i, 1);
			continue;
		}
		var line = input[i].split(" ");
		line[0] = "";
		line[1] += "{";
		input[i] = line.join(" ") + " },";
	}
	input = input.join("\n");
	var sues = eval("({" + input + "})");

	for(var i in sues){
		var thisIsHer = true;
		for(var thing in sues[i]){
			if(typeof(mfcsam[thing]) == "undefined" || mfcsam[thing] != sues[i][thing]){
				thisIsHer = false;
			}
		}
		if(thisIsHer)
			return i;
	}
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].textContent.split("\n");
	var xhr = new XMLHttpRequest();
	xhr.open("GET", "http://adventofcode.com/day/16", false);
	xhr.send(null);
	var parser = new DOMParser();
	var dayPage = parser.parseFromString(xhr.responseText, "text/html");
	var mfcsamTicker = "({" + dayPage.getElementsByTagName("pre")[0].firstChild.textContent.replace(/\n/g, ", ") + "})";
	var mfcsam = eval(mfcsamTicker);

	for(var i in input){
		if(input[i].trim() == ""){
			input.splice(i, 1);
			continue;
		}
		var line = input[i].split(" ");
		line[0] = "";
		line[1] += "{";
		input[i] = line.join(" ") + " },";
	}
	input = input.join("\n");
	var sues = eval("({" + input + "})");

	for(var i in sues){
		var thisIsHer = true;
		for(var thing in sues[i]){
			if(typeof(mfcsam[thing]) != "undefined"){
				switch(thing){
					case "cats":
					case "trees":
						if(sues[i][thing] <= mfcsam[thing])
							thisIsHer = false;
						break;
					case "pomeranians":
					case "goldfish":
						if(sues[i][thing] >= mfcsam[thing])
							thisIsHer = false;
						break;
					default:
						if(sues[i][thing] != mfcsam[thing])
							thisIsHer = false;
						break;
				}
			}
			else
				thisIsHer = false;
		}
		if(thisIsHer)
			return i;
	}
}
