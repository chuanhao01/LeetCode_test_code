with open('data.txt') as f:
    data = []
    for l in f:
        l = l.rstrip()
        data.append(int(l))
    offset = 25
    while offset < len(data):
        cur_num = data[offset]
        nums_set = set(data[offset-25:offset])
        exists = False
        for i in data[offset-25:offset]:
            diff = cur_num - i
            if diff in nums_set:
                exists = True
                break
        if not exists:
            print(cur_num)
            break
        offset += 1