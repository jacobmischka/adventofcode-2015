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
