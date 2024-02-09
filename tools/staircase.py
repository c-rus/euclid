# Project: euclid
# Script: staircase.py
#
# Algorithm implementations for different approaches to computing the set of
# extremal points that define a staircase for a set of points P.
#
# This script is helpful in determing solutions to HW1 problem 5 from UF's
# COT5520 course.


class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return '(' + str(self.x) + ', ' + str(self.y) + ')'
    
    def __eq__(self, rhs):
        return self.x == rhs.x and self.y == rhs.y
    pass


points = [
    Point(0.0, 0.0),
    Point(2.0, 1.0),
    Point(0.0, 5.0),
    Point(1.0, 4.0),
    Point(6.0, 2.0),
    Point(2.0, 1.0),
    Point(4.0, 0.0),
    Point(-1.0, 3.0),
    Point(2.0, -5.0),
]

ideal = [
    Point(0.0, 5.0),
    Point(1.0, 4.0),
    Point(6.0, 2.0),
]


def staircase_a(input):
    '''
    Uses a divide-and-conquer approach to computing the staircase for a given set of points.
    This algorithm has a worst-case time complexity upper-bound of O(n*log(n)).
    '''
    def merge(lhs, rhs):
        i = 0
        j = 0
        merged = []
        # print('conquering:', lhs, rhs)
        floor = None
        while i < len(lhs):
            # there are no more items in the rhs to process
            if j >= len(rhs):
                for l in lhs[i:]:
                    if floor == None or l.y > floor:
                        merged += [l]
                        floor = l.y
                    pass
                break
                
            lp: Point = lhs[i]
            rp: Point = rhs[j]
            # decide which point to pick to be candidate for extremality
            if lp.x > rp.x:
                if floor == None or lp.y > floor:
                    merged += [lp]
                    floor = lp.y
                i += 1
                pass
            elif rp.x > lp.x:
                if floor == None or rp.y > floor:
                    merged += [rp]
                    floor = rp.y
                j += 1
            else:
                if lp.y > rp.y and (floor == None or lp.y > floor):
                    merged += [lp]
                    floor = lp.y
                    i += 1
                    # that point is not extremal
                    j += 1
                elif rp.y > lp.y and (floor == None or rp.y > floor):
                    merged += [rp]
                    floor = rp.y
                    j +=1
                    # that point is not extremal
                    i += 1
                pass
            pass
        # there are still items in the rhs to process
        if j < len(rhs):
            for r in rhs[j:]:
                if floor == None or r.y > floor:
                    merged += [r]
                    floor = r.y
            pass
        # print('merged:', merged)
        return merged
    
    if len(input) > 1:
        # print('dividing:', input)
        subset = merge(staircase_a(input[:int(len(input)/2)]), staircase_a(input[int(len(input)/2):]))
        return subset
    else:
        return input


def staircase_c(points):
    '''
    Finds the set of extremal points for a staircase.
    This algorithm has a worst-case time complexity upper-bound of `O(n*h)`.
    '''
    p: Point
    extremals = []
    remaining = points
    # find the rightmost/highest extremal point O(n)
    prev_exp = None
    while len(remaining) > 0:
        next_remaining = []
        exp: Point = None
        for p in remaining:
            # stop processing this point (its covered by last extremal point)
            if prev_exp is not None and p.x < prev_exp.x and p.y < prev_exp.y:
                continue
            if prev_exp is not None and prev_exp == p:
                continue
            next_remaining += [p]
            if exp is None or p.x > exp.x:
                exp = p
            elif exp is None or p.x == exp.x:
                if exp is None or p.y > exp.y:
                    exp = p
            pass
        prev_exp = exp
        if exp is not None:
            extremals += [exp]
        remaining = next_remaining
        pass

    return extremals[::-1]


result = staircase_a(points)
print('final:', result[::-1])

print(result[::-1] == ideal)

print('---')

result = staircase_c(points)
print('final:', result)
print(result == ideal)