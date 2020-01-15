def find_it(seq):
    count_dict = {}
    for i in seq:
        if str(i) in count_dict:
            count_dict[str(i)] += 1
        else:
            count_dict[str(i)] = 1
    for key, values in count_dict.items():
        if(values % 2 == 1):
            return key

