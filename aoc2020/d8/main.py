with open('data.txt') as f:
    all_instruct = []
    for l in f:
        l = l.rstrip()
        instruct = l.split(' ')
        instruct[1] = int(instruct[1])
        all_instruct.append(instruct)
    prevs = set()
    acc = 0
    i_code = 0
    while True:
        if i_code in prevs:
            break
        prevs.add(i_code)
        instruct = all_instruct[i_code]
        in_op = instruct[0]
        in_num = instruct[1]
        if in_op == 'acc':
            acc += in_num
            i_code += 1
        elif in_op == 'jmp':
            i_code += in_num
        else:
            i_code += 1
    print(acc)

        
