class Solution:
    def maxIncreaseKeepingSkyline(self, grid):
        rows = grid
        columns = self.column(rows)
        new_grid = []
        # setting up new grid with -1
        for i in range(len(grid)):
            row_arr = []
            for j in range(len(grid)):
                row_arr.append(-1)
            new_grid.append(row_arr)
        # for setting largest num in rows
        for i in range(len(rows)):
            largest_num, occurances = self.getMaxAndIndex(rows[i])
            # print(largest_num, occurances)
            for j in occurances:
                new_grid[i][j] = largest_num
        # for setting up the largest num in columns
        for i in range(len(rows)):
           largest_num, occurances = self.getMaxAndIndex(columns[i]) 
        #    print(largest_num, occurances)
           for j in occurances:
               new_grid[j][i] = largest_num
        self.printGrid(grid)
        self.printGrid(new_grid)
        # getting new rows and columns
        total_increase = 0
        new_rows = new_grid
        new_columns = self.column(new_rows)
        for i in range(len(new_rows)):
            
            # print(largest_num)
            for j in range(len(new_rows)):
                largest_num, not_use = self.getMaxAndIndex(new_rows[i])
                if(new_rows[i][j] == -1):
                    # print(i)
                    # print(new_columns[j])
                    column_max, not_use = self.getMaxAndIndex(new_columns[j])
                    if(column_max <= largest_num):
                        largest_num = column_max
                    new_rows[i][j] = largest_num
                    new_grid[i][j] = largest_num
        self.printGrid(new_grid)
        print()
        for i in range(len(new_columns)):
            print(largest_num, i)
            for j in range(len(new_rows)):
                largest_num, not_use = self.getMaxAndIndex(new_columns[i])
                if(new_columns[i][j] == -1):
                    # print(i)
                    # print(new_columns[j])
                    row_max, not_use = self.getMaxAndIndex(new_rows[j])
                    print(row_max, largest_num, i, j)
                    if(row_max <= largest_num):
                        largest_num = row_max
                    new_columns[i][j] = largest_num
                    new_grid[j][i] = largest_num
        self.printGrid(new_grid)
        for i in range(len(new_grid)):
            for j in range(len(new_grid)):
                total_increase += (new_grid[i][j] - grid[i][j])
        return total_increase


    def column(self, rows):
        columns = []
        len_of_side = len(rows)
        for i in range(len_of_side):
            col_arr = []
            for j in range(len_of_side):
                col_arr.append(rows[j][i])
            columns.append(col_arr)
        return columns

    def getMaxAndIndex(self, arr_num):
        temp = arr_num[0]
        occurance_index = []
        for i in range(len(arr_num)):
            # print(temp,arr_num[i])
            # print(arr_num[i] >= temp)
            if(arr_num[i] >= temp):
                if(arr_num[i] == temp):
                    occurance_index.append(i)
                else:
                    occurance_index = [i]
                    temp = arr_num[i]
        return temp, occurance_index

    def printGrid(self, grid):
        for i in grid:
            print(i)

                



