#!/usr/bin/env python3
'''
    Day 2
    http://adventofcode.com/2015/day/2
'''

from itertools import combinations
from utils import get_input_filename
from functools import reduce

DAY_NUM = 2

# This does not feel very good
def part1():
    wrapping_paper = 0
    with open(get_input_filename(DAY_NUM)) as f:
        for line in f:
            sides = [reduce(lambda x, y: int(x) * int(y), combination) for combination in combinations(line.split('x'), 2)]
            wrapping_paper += min(sides)
            for side in sides:
                wrapping_paper += side * 2
    return wrapping_paper



if __name__ == '__main__':
    print(part1())
    # print(part2())
