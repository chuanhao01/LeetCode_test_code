val = 0
with open('data.txt') as f:
    for l in f:
        l = l.split(': ')
        context = l[0].split(' ')
        c_range = context[0].split('-')
        c_min = int(c_range[0])
        c_max = int(c_range[1])
        c_word = context[1]
        password = l[1].rstrip()
        count = 0
        for c in password:
            if c == c_word:
                count += 1
        if c_min <= count <= c_max:
            val += 1
print(val)