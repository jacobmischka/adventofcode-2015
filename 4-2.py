#!/usr/bin/env python3

import hashlib

puzzle_input = "ckczppom"
output_hash = "1"
num = 0
while output_hash[:6] != "000000":
	num += 1
	output_hash = hashlib.md5((puzzle_input + str(num)).encode("utf-8")).hexdigest()

print(output_hash + "\n")
print(num)
