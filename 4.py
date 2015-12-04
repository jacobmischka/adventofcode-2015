#!/usr/bin/env python3
'''
	Day 4
	http://adventofcode.com/day/4
'''
import hashlib

puzzle_input = "ckczppom"

def part1():
	output_hash = "1"
	num = 0
	while output_hash[:5] != "00000":
		num += 1
		output_hash = hashlib.md5((puzzle_input + str(num)).encode("utf-8")).hexdigest()

	print(num)

def part2():
	output_hash = "1"
	num = 0
	while output_hash[:6] != "000000":
		num += 1
		output_hash = hashlib.md5((puzzle_input + str(num)).encode("utf-8")).hexdigest()

	print(num)

if __name__ == "__main__":
	part1()
	part2()
