class Solution:
    def judgeCircle(self, moves):
        direction_map = {
            'L' : (-1, 0),
            'U' : (0, 1),
            'R' : (1, 0),
            'D' : (0, -1),
        }
        final_position = (0, 0)
        for i in moves:
            final_position = self.addMoves(final_position, direction_map[i])
        if (final_position == (0,0)):
            return True
        else:
            return False
        

    def addMoves(self, tuple_1, tuple_2):
        rtr_tuple = ((tuple_1[0] + tuple_2[0]), (tuple_1[1] + tuple_2[1]))
        return rtr_tuple
    
