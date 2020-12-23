
with open('data.txt') as f:
    s = 0
    cur_set = None
    for l in f:
        l = l.rstrip()
        if l == '':
            # Case where we move on to new grp
            s += len(cur_set)
            cur_set = None
            continue
        new_set = set([i for i in l])
        if cur_set is None:
            # If we are on the first person in the grp
            cur_set = new_set
        else:
            cur_set = cur_set.intersection(new_set)
    s += len(cur_set)

print(s)
