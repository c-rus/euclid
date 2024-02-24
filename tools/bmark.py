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

# X, Y ~ UNIFORM[0, 100]
# kd_tree = [
#     305.65 * 0.001,
#     1.3954 * 1,
#     7.2284 * 1,
#     33.611 * 1,
#     53.944 * 1,
# ]

# range_tree = [
#     126.81 * 0.001,
#     1.8521 * 1,
#     8.9364 * 1,
#     22.842 * 1,
#     27.417 * 1,
# ]

# X, Y ~ NORM(mean=0, var=81)
kd_tree = [
    450.92 * 0.001,
    1.4431 * 1,
    9.1343 * 1,
    43.198 * 1,
    64.845 * 1,
]

range_tree = [
    306.14 * 0.001,
    2.0150 * 1,
    10.682 * 1,
    23.461 * 1,
    28.447 * 1,
]

# fn_loglog = [math.log2(n)**2 for n in sample_sizes]
# fn_sqrt = [math.sqrt(n) for n in sample_sizes]

# plt.plot(sample_sizes, fn_sqrt, label='${n^{1/2}}$')
# plt.plot(sample_sizes, fn_loglog, label='${\log_2^{2}(n)}$')

# create the plot along with the proper formatting
plt.plot(sample_sizes, kd_tree, label='kd-tree')
plt.plot(sample_sizes, range_tree, label='range-tree')

plt.title('Range Query Time Analysis (Normal Distribution)')
plt.xscale('log')

plt.xlabel('N (number of points)')
# plt.ylabel('Execution time')
plt.ylabel('Execution time ('+str(units)+')')

plt.legend()
plt.show()


# uniform: X, Y ~ U[0, 100]
# normal: X, Y ~ N(mu=0, var=81)

# kd_tree = [
#     _ * 0.001,
#     _ * 1,
#     _ * 1,
#     _ * 1,
#     _ * 1,
# ]

# range_tree = [
#     _ * 0.001,
#     _ * 1,
#     _ * 1,
#     _ * 1,
#     _ * 1,
# ]