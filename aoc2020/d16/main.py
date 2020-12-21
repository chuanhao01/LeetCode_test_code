def check_ranges(num, ranges):
    '''
    Helper func to check a num against all the ranges
    If it is within any range, return True
    If not within any range, return False

    Parameters
    -------------
    num : int
        The input number to check
    ranges : array
        Array of format [(low, high)]
        Where low and high are inclusive
    '''
    for (l, h) in ranges:
        if l <= num <= h:
            return True
    return False

# print(check_ranges(7, [(1, 3), (5, 7), (6, 11), (33, 44), (13, 40), (45, 50)]))
# print(check_ranges(47, [(1, 3), (5, 7), (6, 11), (33, 44), (13, 40), (45, 50)]))
# print(check_ranges(4, [(1, 3), (5, 7), (6, 11), (33, 44), (13, 40), (45, 50)]))

ranges = []
with open('ranges.txt') as f:
    for l in f:
        l = l.rstrip()
        raw_range = l.split(': ')[1]
        range_str = raw_range.split(' or ')
        # print(range_str)
        for r_raw in range_str:
            r = r_raw.split('-')
            [l, h] = r
            l = int(l)
            h = int(h)
            ranges.append((l, h))

# print(ranges)
invalid = 0

with open('input.txt') as f:
    for l in f:
        l = l.rstrip()
        for raw_num in l.split(','):
            num = int(raw_num)
            if not check_ranges(num, ranges):
                invalid += num

print(invalid)