// Day 11
// http://adventofcode.com/day/11
var input = "hepxcrrq";

function part1(){
	var password = input;
	do {
		password = incrementPassword(password);
	} while(!isValidPassword(password));
	return password;
}

function part2(){
	var password = input;
	var numIterations = 0;
	do {
		password = incrementPassword(password);
		if(isValidPassword(password))
			numIterations++;
	} while(numIterations < 2);
	return password;
}

function isValidPassword(password){
	if(password.indexOf("i") != -1 || password.indexOf("o") != -1 || password.indexOf("l") != -1)
		return false;
	var numPairs = 0, numStraights = 0;

	if(password[0] == password[1])
		numPairs++;
	for(var i = 2; i < password.length; i++){
		if(password[i] == password[i-1] && password[i-1] != password[i-2])
			numPairs++;
		if(password[i].charCodeAt()-1 == password[i-1].charCodeAt() &&
				password[i-1].charCodeAt()-1 == password[i-2].charCodeAt())
			numStraights++;
	}
	return (numPairs >= 2 && numStraights >= 1);
}

function incrementPassword(password){
	password = password.split("");
	var index = password.length-1;
	while(index >= 0){
		if(password[index] < "z"){
			password[index] = String.fromCharCode(password[index].charCodeAt()+1);
			return password.join("");
		}
		else{
			password[index] = "a";
			index--;
		}
	}
	return password.join("");
}
