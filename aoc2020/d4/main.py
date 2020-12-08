total = 0
valids = 0
with open('data.txt') as f:
    cur_pass = set()
    fields = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    for l in f:
        l = l.rstrip()
        if l == '':
            if fields.issubset(cur_pass):
                valids += 1
            total += 1
            cur_pass = set()
            continue
        f = [i.split(':')[0] for i in l.split(' ')]
        for i in f:
            cur_pass.add(i)
    if fields.issubset(cur_pass):
        valids += 1
print(valids)
print(total)
        