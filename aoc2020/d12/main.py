def cal_new_head(direction, num, cur_head):
    num = num//90
    if direction == 'L':
        num = -num
    return (cur_head + 4 + num) % 4

def cal_new_x_y(direction, num, x, y):
    if direction == 'N':
        y += num
    elif direction == 'S':
        y -= num
    elif direction == 'E':
        x += num
    elif direction == 'W':
        x -= num
    return x, y

with open('data.txt') as f:
    x = 0
    y = 0
    dirs = ['N', 'E', 'S', 'W']
    cur_head = 1
    for l in f:
        l = l.rstrip()
        direction = l[0]
        num = int(l[1:])
        # print(direction, num)
        if direction in dirs:
            x, y = cal_new_x_y(direction, num, x, y)
        elif direction in {'L', 'R'}:
            cur_head = cal_new_head(direction, num, cur_head)
        elif direction == 'F':
            x, y = cal_new_x_y(dirs[cur_head], num, x, y)
    print(x, y)
    print(abs(x)+abs(y))
