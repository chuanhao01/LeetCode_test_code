import json

class Node:
    def __init__(self, num, key):
        self.num = num
        self.key = key

def parse_s(s):
    return s[0], s[2:]

bag_map = {}

with open('data.txt') as f:
    for l in f:
        l = l.rstrip()
        l = l[:-1]
        l = l.replace(' bags', '')
        l = l.replace(' bag', '')
        # data = l.split(' contain ')
        # print(data)

        origin, nodes = l.split(' contain ')
        if nodes == 'no other':
            # Base case
            continue
        if origin not in bag_map:
            bag_map[origin] = []
        for n in nodes.split(', '):
            num, key = parse_s(n)
            if key not in bag_map:
                bag_map[key] = []
            bag_map[key].append(origin)

def get_num(s, visited = set()):
    if s in visited:
        return 0
    else:
        visited.add(s)
    acc = 1
    for neighbour in bag_map[s]:
        acc += get_num(neighbour)
    return acc

    

# print(json.dumps(bag_map, sort_keys=True, indent=2))
print(get_num('shiny gold') - 1)
    
