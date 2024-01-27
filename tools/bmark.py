# Project: euclid
# Script: bmark.py
#
# Generate a plot comparing computational runtimes for algorithms
# using matplotlib.

from matplotlib import pyplot as plt

units = 'µs'

sample_sizes = [
    100,
    200,
    300,
    400,
    500,
    600,
    700,
    800,
    900,
    1000,
]

brute_force = [
    147.9 * 1,
    633.80 * 1,
    1.4585 * 1_000,
    2.9887 * 1_000,
    4.9688 * 1_000,
    4.9627 * 1_000,
    7.6752 * 1_000,
    11.942 * 1_000,
    14.429 * 1_000,
    15.756 * 1_000,
]

upper_lower = [
    5.1325 * 1,
    8.9046 * 1,
    15.426 * 1,
    33.244 * 1,
    37.911 * 1,
    48.199 * 1,
    55.925 * 1,
    67.040 * 1,
    81.943 * 1,
    104.02 * 1,
]

# create the plot along with the proper formatting
plt.plot(sample_sizes, brute_force, label='brute-force')
plt.plot(sample_sizes, upper_lower, label='upper-lower hull')

plt.title('Convex Hull Benchmark Analysis')

plt.yscale('log')

plt.xlabel('N (number of points)')
plt.ylabel('Runtime ('+str(units)+')')

plt.legend()
plt.show()
