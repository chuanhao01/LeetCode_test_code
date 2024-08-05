from typing import List

class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        q = [(sr, sc)]
        s = set()
        cc = image[sr][sc]
        while len(q) > 0:
            c = q.pop()
            s.add(c)
            if image[c[0]][c[1]] == cc:
                image[c[0]][c[1]] = color
                for (y, x) in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                    ny = c[0] + y
                    nx = c[1] + x
                    if (ny, nx) in s:
                        continue
                    if ny < 0 or ny >= len(image) or nx < 0 or nx >= len(image[0]):
                        continue
                    q.append((ny, nx))
        return image
