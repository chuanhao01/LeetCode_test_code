num_set = set()
nums = []
with open('data.txt') as f:
    for l in f:
        num = int(l)
        num_set.add(num)
        nums.append(num)
    for i in nums:
        if (2020 - i) in num_set:
            print(i * (2020 -i))