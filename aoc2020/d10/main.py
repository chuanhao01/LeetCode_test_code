with open('data.txt') as f:
    nums = []
    for l in f:
        l = l.rstrip()
        num = int(l)
        nums.append(num)
    nums.sort()
    # print(nums)
    cur = 0
    diffs_1 = 0
    diffs_3 = 1 # Plus 1 for the final diff
    for num in nums:
        diff = num - cur
        if diff == 1:
            diffs_1 += 1
        elif diff == 3:
            diffs_3 += 1
        cur = num
    print(diffs_1 * diffs_3)