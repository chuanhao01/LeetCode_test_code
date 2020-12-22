val = 0
with open('data.txt') as f:
    for l in f:
        l = l.split(': ')
        context = l[0].split(' ')
        c_range = context[0].split('-')
        c_min = int(c_range[0]) - 1
        c_max = int(c_range[1]) - 1
        c_word = context[1]
        password = l[1].rstrip()
        count = 0
        for i in [c_min, c_max]:
            if password[i] == c_word:
                count += 1
        if count == 1:
            val += 1
print(val)