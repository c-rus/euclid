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
from matplotlib import collections  as mc
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
    Returns a 2-D tuple containing the x and y coordinates.

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


def parse_rectangle(fpath: str):
    '''
    Returns a 2-D tuple containing the x and y coordinates.

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
    x = [x[0], x[1], x[1], x[0], x[0]]
    y = [y[0], y[0], y[1], y[1], y[0]]

    return (x, y)


def parse_segments(fpath: str):
    '''
    Returns a list of disconnected line segments to plot.
    '''
    segs = []
    with open(fpath, 'r') as fd:
        for line in fd.readlines():
            p0, q0 = line.strip().split(',')
            p_x, p_y = p0.split()
            q_x, q_y = q0.split()
            segs += [((float(p_x), float(p_y)), (float(q_x), float(q_y)))]
        pass
    return segs


plt.title('Graph')
plt.xlabel('X')
plt.ylabel('Y')

colors_count = 0

for path in sys.argv[1:]:
    if path.count('point') > 0:
        plt.scatter(*parse_points(path), s=(20 - colors_count*10))
        colors_count += 1
    elif path.count('poly') > 0:
        plt.plot(*parse_polygon(path), color='red')
    elif path.count('stair') > 0:
        plt.step(*parse_points(path), where='post', color='red')
    elif path.count('range') > 0:
        plt.plot(*parse_rectangle(path), color='red')
    elif path.count('segments') > 0:
        for seg in parse_segments(path):
            print(seg)
            plt.plot((seg[0][0], seg[1][0]), (seg[0][1], seg[1][1]), color='black')
            plt.scatter(*seg[0], color='black')
            plt.scatter(*seg[1], color='black')
    elif path.count('box') > 0:
        plt.plot(*parse_rectangle(path), color='black')
    pass

plt.show()
