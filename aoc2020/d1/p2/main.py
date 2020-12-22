num_set = {}
nums = []
with open('data.txt') as f:
    for l in f:
        num = int(l)
        nums.append(num)
    for i in nums:
        for j in nums:
            if i == j:
                continue
            s = i + j
            num_set[2020-s] = (i, j)
    for i in nums:
        if i in num_set:
            other_nums = num_set[i]
            print(i * other_nums[0] * other_nums[1])
