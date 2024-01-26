# Project: euclid
# Script: plot.py
#
# This script plots a series of points for visualization.

from matplotlib import pyplot as plt

POINTS = 'data/points.txt'
POLYGON = 'data/hull.txt'


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

plt.scatter(*parse_points(POINTS), s=10)
plt.plot(*parse_polygon(POLYGON), color='red')

plt.show()
