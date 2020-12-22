def get_count(data, x_cap, y_cap, x_add, y_add):
    x = 0
    y = 0
    count = 0
    while y < y_cap:
        if data[y][x] == '#':
            count += 1
        x += x_add
        x %= x_cap
        y += y_add
    return count

with open('data.txt') as f:
    x_cap = 0
    y_cap = 0
    data = []
    for l in f:
        l = l.rstrip()
        x_cap = len(l)
        data.append(l)
    y_cap = len(data)
    print(x_cap, y_cap)
    # x = 0
    # y = 0
    # count = 0
    # while y < y_cap:
    #     if data[y][x] == '#':
    #         count += 1
    #     x += 3
    #     x %= x_cap
    #     y += 1
    # print(count)
    prod = 1
    prod *= get_count(data, x_cap, y_cap, 1, 1)
    prod *= get_count(data, x_cap, y_cap, 3, 1)
    prod *= get_count(data, x_cap, y_cap, 5, 1)
    prod *= get_count(data, x_cap, y_cap, 7, 1)
    prod *= get_count(data, x_cap, y_cap, 1, 2)
    print(prod)

