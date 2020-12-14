import copy
class Data:
    def __init__(self, data):
        self.data = data
        self.max_row = len(data)
        self.max_col = len(data[0])
    
    def get_value(self, row, col):
        if row < 0 or row >= self.max_row:
            return '.'
        if col < 0 or col >= self.max_col:
            return '.'
        return self.data[row][col]
    
    def set_value(self, row, col, val):
        if row < 0 or row >= self.max_row:
            return None
        if col < 0 or col >= self.max_col:
            return None
        self.data[row][col] = val

    
    def run_algo(self):
        '''
        Returns a new Data obj
        '''
        new_data = self.dup()
        for row in range(self.max_row):
            for col in range(self.max_col):
                num_occ = 0
                for check_row in range(row-1, row+2):
                    for check_col in range(col-1, col+2):
                        if check_row == row and check_col == col:
                            continue
                        check_val = self.get_value(check_row, check_col)
                        if check_val == '#':
                            num_occ += 1
                val = self.get_value(row, col)
                if val == 'L':
                    # If empty
                    # No occ
                    if num_occ == 0:
                        # new_data.set_value(row, col, '#')
                        # print(self.data[row][col], new_data.data[row][col])
                        new_data.data[row][col] = '#'
                        # print(self.data[row][col], new_data.data[row][col])
                        # print()
                elif val == '#':
                    # If occ
                    # 4 or more
                    if num_occ >= 4:
                        # new_data.set_value(row, col, 'L')
                        new_data.data[row][col] = 'L'

        # print(self.data[0])
        # print(new_data.data[0])
        return new_data
    
    def dup(self):
        d = copy.deepcopy(self.data)
        new_data = Data(d)
        return new_data


    def check_data(self, past_data):
        '''
        Check if the current data is the same as the prev
        '''
        for cur_row, past_row in zip(self.data, past_data.data):
            for cur, past in zip(cur_row, past_row):
                if cur != past:
                    # If no change
                    return False
        # If all the same no change
        return True
    
    def get_occ(self):
        occ_num = 0
        for row in range(self.max_row):
            for col in range(self.max_col):
                if self.get_value(row, col) == '#':
                    occ_num += 1
        return occ_num


with open('data.txt') as f:
    data = []
    for l in f:
        l = l.rstrip()
        data.append([c for c in l])
    cur_data = Data(data)
    while True:
        next_data = cur_data.run_algo()
        # print(cur_data.data[0])
        # print(next_data.data[0])
        if cur_data.check_data(next_data):
            print(cur_data.get_occ())
            break
        cur_data = next_data


