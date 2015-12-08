// Day 7
// http://adventofcode.com/day/7

function part1(){
	var input = document.getElementsByTagName("pre")[0].innerText.split("\n");
	var output = 0, input1 = 2, operation = 3, input2 = 4;
	var wires = {};
	wires.set = function(key, value){
		wires[key] = 0x0000ffff & value;
	}
	while(input.length > 0){
		for(var i = 0; i < input.length; i++){
			if(input[i].trim() == ""){
				input.splice(i, 1);
				continue;
			}
			var input1Value, input2Value, outputValue;
			var command = input[i].split(" ");
			command = command.reverse();

			input1Value = isNaN(parseInt(command[input1])) ? wires[command[input1]] : parseInt(command[input1])
			if(typeof(input1Value) == "undefined")
				continue;
			if(command.length == 3){
				wires.set(command[output], input1Value);
				console.log(input[i]);
				input.splice(i--, 1);
			}
			else{
				input2Value = isNaN(parseInt(command[input2])) ? wires[command[input2]] : parseInt(command[input2])
				if(command.length == 5 && typeof(input2Value) == "undefined")
					continue;
				switch(command[operation]){
					case "NOT":
						outputValue = ~input1Value;
						break;
					case "OR":
						outputValue = input2Value | input1Value;
						break;
					case "AND":
						outputValue = input2Value & input1Value;
						break;
					case "LSHIFT":
						outputValue = input2Value << input1Value;
						break;
					case "RSHIFT":
						outputValue = input2Value >> input1Value;
						break;
				}
				wires.set(command[output], outputValue);
				console.log(input[i]);
				input.splice(i--, 1);
			}
		}
	}

	return wires;
}

function part2(){
	var input = document.getElementsByTagName("pre")[0].innerText.split("\n");
	var output = 0, input1 = 2, operation = 3, input2 = 4;
	var wires = {b: 16076};
	wires.set = function(key, value){
		wires[key] = 0x0000ffff & value;
	}
	while(input.length > 0){
		for(var i = 0; i < input.length; i++){
			if(input[i].trim() == ""){
				input.splice(i, 1);
				continue;
			}
			var input1Value, input2Value, outputValue;
			var command = input[i].split(" ");
			command = command.reverse();

			input1Value = isNaN(parseInt(command[input1])) ? wires[command[input1]] : parseInt(command[input1])
			if(typeof(input1Value) == "undefined")
				continue;
			if(command.length == 3){
				if(command[output] != "b")
					wires.set(command[output], input1Value);
				console.log(input[i]);
				input.splice(i--, 1);
			}
			else{
				input2Value = isNaN(parseInt(command[input2])) ? wires[command[input2]] : parseInt(command[input2])
				if(command.length == 5 && typeof(input2Value) == "undefined")
					continue;
				switch(command[operation]){
					case "NOT":
						outputValue = ~input1Value;
						break;
					case "OR":
						outputValue = input2Value | input1Value;
						break;
					case "AND":
						outputValue = input2Value & input1Value;
						break;
					case "LSHIFT":
						outputValue = input2Value << input1Value;
						break;
					case "RSHIFT":
						outputValue = input2Value >> input1Value;
						break;
				}
				wires.set(command[output], outputValue);
				console.log(input[i]);
				input.splice(i--, 1);
			}
		}
	}

	return wires;
}
