#!/usr/bin/env python3
'''
    Day 1
    http://adventofcode.com/2015/day/1
'''

from utils import get_input_filename

DAY_NUM = 1

def part1():
    level = 0
    with open(get_input_filename(DAY_NUM)) as f:
        for line in f:
            for c in line:
                if c == '(':
                    level += 1
                elif c == ')':
                    level -= 1
    return level

def part2():
    level = 0
    position = 0
    with open(get_input_filename(DAY_NUM)) as f:
        for line in f:
            for c in line:
                position += 1
                if c == '(':
                    level += 1
                elif c == ')':
                    level -= 1
                if level < 0:
                    return position


if __name__ == '__main__':
    print(part1())
    print(part2())
