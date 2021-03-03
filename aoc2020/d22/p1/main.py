import collections
p1 = collections.deque([8, 19, 46, 11, 36, 10, 35, 9, 24, 22, 50, 1, 34, 7, 18, 28, 3, 38, 43, 2, 6, 42, 23, 12, 20])
p2 = collections.deque([39, 27, 44, 29, 5, 48, 30, 32, 15, 31, 14, 21, 49, 17, 45, 47, 16, 26, 33, 25, 13, 41, 4, 40, 37])

while True:
    if len(p1) == 0 or len(p2) == 0:
        break
    p1_card = p1.popleft()
    p2_card = p2.popleft()
    if p1_card > p2_card:
        p1.append(p1_card)
        p1.append(p2_card)
    else:
        p2.append(p2_card)
        p2.append(p1_card)

p = p1 if len(p2) == 0 else p2

acc = 0
for i, s in enumerate(reversed(p)):
    acc += (i+1) * s
print(acc)



