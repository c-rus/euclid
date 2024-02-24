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
    315.47 * 0.001,
    1.4884 * 1,
    7.7223 * 1,
    35.541 * 1,
    64.157 * 1,
]

range_tree = [
    165.24 * 0.001,
    1.0399 * 1,
    4.2857 * 1,
    22.597 * 1,
    36.846 * 1,
]

# fn_loglog = [math.log2(n)**2 for n in sample_sizes]
# fn_sqrt = [math.sqrt(n) for n in sample_sizes]

# plt.plot(sample_sizes, fn_sqrt, label='${n^{1/2}}$')
# plt.plot(sample_sizes, fn_loglog, label='${\log_2^{2}(n)}$')

# create the plot along with the proper formatting
plt.plot(sample_sizes, kd_tree, label='kd-tree')
plt.plot(sample_sizes, range_tree, label='range-tree')

plt.title('Range Query Big-O Analysis')
plt.xscale('log')

plt.xlabel('N (number of points)')
# plt.ylabel('Execution time')
plt.ylabel('Execution time ('+str(units)+')')

plt.legend()
plt.show()
