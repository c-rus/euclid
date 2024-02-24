# Project: euclid
# Script: sample.py
#
# This script generates a number of 2-dimensional points from a uniform
# distribution. The x and y coordinates are independent of each other.

import random
import sys
from scipy import stats
import math

args = sys.argv


RANGE_L = 100
RANGE_H = 0

random.seed(999)

RANGE_FILE = 'data/range.txt'


def rand_unique_points(low, high, n, dist: str, dtype):
    group = set()
    X = None
    Y = None
    if dist == 'norm':
        X = stats.norm(loc=0, scale=math.sqrt(81))
        Y = stats.norm(loc=0, scale=math.sqrt(81))

    while True:
        if len(group) >= n:
            break
        if dist == 'uni':
            # central to (0, 0)
            with open(RANGE_FILE, 'w') as f: f.write('1.0 4.0\n9.0 19.0\n')
            x = random.uniform(low, high)
            y = random.uniform(low, high)
            pass
        if dist == 'norm':
            with open(RANGE_FILE, 'w') as f: f.write('10.0 10.0\n20.0 20.0\n')
            x = X.rvs(size=1)[0]
            y = Y.rvs(size=1)[0]
            pass
        if dtype == int:
            x = int(x)
            y = int(y)
        elif dtype == float:
            x = round(x, 6)
            y = round(y, 6)
        
        if (x, y) in group:
            continue
        group.add((x, y))
        print(x, y)
        pass


def rand_points(low, high, n):
    for _ in range(0, n):
        x = random.uniform(low, high)
        y = random.uniform(low, high)
        x = round(x, 2)
        y = round(y, 2)
        print(x, y)
    pass

# type of input (points, polygon (simple))
FORM = str(args[1])
# number of points to generate
N = int(args[2])

DIST = str(args[3]) if len(args) > 3 else None

if FORM == 'points':
    rand_points(-512, 511, N)
    pass
elif FORM == 'hw3p4':
    # generate uniform points over range
    # print('1.0 1.0')
    # print('2.0 2.0')
    # print('3.0 3.0')
    # print('4.0 4.0')
    # print('5.0 5.0')
    # print('6.0 6.0')
    rand_unique_points(RANGE_L, RANGE_H, N, DIST, dtype=float)
    pass
elif FORM == 'polygon':
    exit(101)
    pass
elif FORM == 'points1':
    print('0.0 0.0')
    print('2.0 10.0')
    print('5.0 1.0')

    # interior points
    print('2.0 4.0')
    print('3.0 2.0')
    pass
elif FORM == 'points2':
    print('0.0 0.0')
    print('6.0 0.0')
    print('5.0 5.0')
    print('3.0 0.0')
    print('4.0 0.0')
    print('1.0 5.0')

    # interior points
    print('2.0 2.0')
    pass
else:
    print('error: Unknown input type: '+str(FORM))
    exit(101)