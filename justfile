# Project: euclid
# Script: justfile
#
# A collection of shortcuts to run a series of commands together.

convex ALGO PSET:
    cargo b
    python tools/sample.py {{PSET}} 100 > data/points.txt
    ./target/debug/euclid {{ALGO}} data/points.txt data/poly_hull.txt
    python tools/plot.py data/points.txt data/poly_hull.txt


stairs PSET:
    cargo b
    python tools/sample.py {{PSET}} 10 > data/points.txt
    ./target/debug/euclid stairs data/points.txt data/stair.txt
    python tools/plot.py data/points.txt data/stair.txt


# STRUCT is either: "kd" or "range"
hw3p4 STRUCT N DIST:
    cargo b
    python tools/sample.py hw3p4 {{N}} {{DIST}} > data/points.txt
    ./target/debug/euclid {{STRUCT}} data/points.txt data/range.txt data/points_out.txt
    python tools/plot.py data/points.txt data/range.txt data/points_out.txt

hw3p4-uni:
    python tools/sample.py hw3p4 100 uni > data/points100uni.txt
    python tools/sample.py hw3p4 1000 uni > data/points1000uni.txt
    python tools/sample.py hw3p4 65536 uni > data/points65536uni.txt
    python tools/sample.py hw3p4 10000 uni > data/points10000uni.txt

hw3p4-norm:
    python tools/sample.py hw3p4 100 norm > data/points100norm.txt
    python tools/sample.py hw3p4 1000 norm > data/points1000norm.txt
    python tools/sample.py hw3p4 1000 norm > data/points10000norm.txt
    python tools/sample.py hw3p4 65536 norm > data/points65536norm.txt
    python tools/sample.py hw3p4 100000 norm > data/points100000norm.txt

plot FILE:
    python tools/plot.py {{FILE}}