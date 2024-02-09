# Project: euclid
# Script: sample.py
#
# This script generates a number of 2-dimensional points from a uniform
# distribution. The x and y coordinates are independent of each other.

import random
import sys

args = sys.argv


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

if FORM == 'points':
    rand_points(-512, 511, N)
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