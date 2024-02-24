# Project: euclid
# Script: bmark.py
#
# Generate a plot comparing computational runtimes for algorithms
# using matplotlib.

from matplotlib import pyplot as plt
import math


units = 'µs'

sample_sizes = [
    100,
    1_000,
    10_000,
    65_536,
    100_000,
]

kd_tree = [
    305.65 * 0.001,
    1.3954 * 1,
    7.2284 * 1,
    33.611 * 1,
    53.944 * 1,
]

range_tree = [
    126.81 * 0.001,
    1.8521 * 1,
    8.9364 * 1,
    22.842 * 1,
    27.417 * 1,
]

# fn_loglog = [math.log2(n)**2 for n in sample_sizes]
# fn_sqrt = [math.sqrt(n) for n in sample_sizes]

# plt.plot(sample_sizes, fn_sqrt, label='${n^{1/2}}$')
# plt.plot(sample_sizes, fn_loglog, label='${\log_2^{2}(n)}$')

# create the plot along with the proper formatting
plt.plot(sample_sizes, kd_tree, label='kd-tree')
plt.plot(sample_sizes, range_tree, label='range-tree')

plt.title('Range Query Time Analysis')
plt.xscale('log')

plt.xlabel('N (number of points)')
# plt.ylabel('Execution time')
plt.ylabel('Execution time ('+str(units)+')')

plt.legend()
plt.show()
