# Project: euclid
# Script: sample.py
#
# This script generates a number of 2-dimensional points from a uniform
# distribution. The x and y coordinates are independent of each other.

import random

SAMPLES = 100

LIMIT_LOWER = -512
LIMIT_UPPER = 511

def c1():
    print('0.0 0.0')
    print('2.0 10.0')
    print('5.0 1.0')

    # interior points
    print('2.0 4.0')
    print('3.0 2.0')
    pass


def d1():
    print('0.0 0.0')
    print('6.0 0.0')
    print('5.0 5.0')
    print('3.0 0.0')
    print('4.0 0.0')
    print('1.0 5.0')

    # interior points
    print('2.0 2.0')
    pass


def rand():
    for _ in range(0, SAMPLES):
        x = random.uniform(LIMIT_LOWER, LIMIT_UPPER)
        y = random.uniform(LIMIT_LOWER, LIMIT_UPPER)
        x = round(x, 2)
        y = round(y, 2)
        print(x, y)
    pass


# c1()
# d1()
rand()