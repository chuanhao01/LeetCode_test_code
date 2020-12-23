def get_num(i, arr):
    cur = arr[i]
    try:
        cur = int(cur)
        if i == len(arr) - 1:
            return cur
        op = arr[i+1]
        if op == '+':
            return cur + get_num(i+2, arr)
        elif op == '*':
            return cur * get_num(i+2, arr)
        elif op == ')':
            if i + 1 == len(arr) - 1:
                return cur
            
    except:
        if cur == '(':
            return get_num(i+1, arr)



s = 0
with open('data.txt') as f:
    for l in f:
        l = l.rstrip()
        l = l.replace(' ', '')
        eqn = [i for i in l]
        print(get_num(0, eqn))


