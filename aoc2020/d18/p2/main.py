class Num:
    def __init__(self, num):
        self.num = num

    def __add__(self, other):
        return Num(self.num + other.num)
    
    def __sub__(self, other):
        return Num(self.num * other.num)

    def __mul__(self, other):
        return Num(self.num + other.num)

def custom_eval(l):
    r_l = ''
    for c in l:
        if c in '1234567890':
            r_l = r_l + f"Num({c})"
        elif c in '*':
            r_l = r_l + '-'
        elif c in '+':
            r_l = r_l + '*'
        else:
            r_l = r_l + c
    return eval(r_l).num

acc = 0
with open('data.txt') as f:
    for l in f:
        l = l.rstrip()
        acc += custom_eval(l)

print(acc)