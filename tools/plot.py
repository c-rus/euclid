# Project: euclid
# Script: plot.py
# Usage: python plot.py [FILE...]
#
# This script plots a series of points for visualization.
#
# The script determines how to graph the set of points based on the clues from
# the file's name.
# - "point": scatter plot
# - "poly": draw as a series of connected points in order
# - "stair": draw as a step function

from matplotlib import pyplot as plt
import sys


def parse_points(fpath: str):
    '''
    Returns a 2-D tuple containing the x and y coordinates.

    The output of this function can be the input to `pyplot.scatter(...)`.
    '''
    x = []
    y = []
    with open(fpath, 'r') as fd:
        for line in fd.readlines():
            t0, t1 = line.strip().split()
            x += [float(t0)]
            y += [float(t1)]
        pass

    return (x, y)


def parse_polygon(fpath: str):
    '''
    Returns a 2-D tupel containing the x and y coordinates.

    The output of this function can be the input to `pyplot.plot(...)`.
    '''
    x = []
    y = []
    with open(fpath, 'r') as fd:
        for line in fd.readlines():
            t0, t1 = line.strip().split()
            x += [float(t0)]
            y += [float(t1)]
        pass
    # produce the final connections to enclose the polygon
    x += [x[0]]
    y += [y[0]]

    return (x, y)


plt.title('Graph')
plt.xlabel('X')
plt.ylabel('Y')

for path in sys.argv[1:]:
    if path.count('point') > 0:
        plt.scatter(*parse_points(path), s=10)
    elif path.count('poly') > 0:
        plt.plot(*parse_polygon(path), color='red')
    elif path.count('stair') > 0:
        plt.step(*parse_points(path), where='post', color='red')
    pass

plt.show()
