puzzle_input = [2,0,1,7,4,14,18]
num_map = {}
num_counts = {}
count = 1
cur_num = None
for i in puzzle_input:
    num_map[i] = count
    num_counts[i] = 1
    cur_num = i
    count += 1

while count<=2020:
    print(cur_num)
    # Update prev num to curent num
    if num_counts[cur_num] == 1:
        cur_num = 0
    elif num_counts[cur_num] > 1:
        diff = count - 1 - num_map[cur_num]
        num_map[cur_num] = count - 1
        cur_num = diff

    # Checking if cur num has been seen b4
    if cur_num not in num_counts:
        num_counts[cur_num] = 1
        num_map[cur_num] = count
    else:
        num_counts[cur_num] += 1
    
    count += 1
print(cur_num)

# 2, 0, 1, (0), ()
# 1, 2, 3,  4,   5