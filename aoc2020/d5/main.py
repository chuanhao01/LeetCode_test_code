def get_seat_num(seat_code):
    row_code = seat_code[:7]
    col_code = seat_code[7:]
    row_min = 0
    row_max = 128
    for i in row_code:
        new_num = (row_min + row_max)/2
        if i == 'B':
            row_min = new_num
        else:
            row_max = new_num
    row = int(row_max - 1)
    col_min = 0
    col_max = 8
    for i in col_code:
        new_num = (col_min + col_max)/2
        if i == 'R':
            col_min = new_num
        else:
            col_max = new_num
    col = int(col_max - 1)
    # print(row, col)
    return ((row * 8) + col)

# print(get_seat_num('FFFBBBFRRR'))

max_num = 0
with open('data.txt') as f:
    for l in f:
        l = l.rstrip()
        max_num = max(max_num, get_seat_num(l))
print(max_num)