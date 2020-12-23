with open('data.txt') as f:
    s = 0
    c_set = set()
    for l in f:
        l = l.rstrip()
        if l == '':
            s += len(c_set)
            c_set = set()
            continue
        for c in l:
            c_set.add(c)
    s += len(c_set)
    print(s)