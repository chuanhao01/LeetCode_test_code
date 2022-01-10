import queue


def solve_challenge(data: str):
    f_data = []
    for row in data.split("\n"):
        rr = []
        for d in row:
            rr.append(int(d))
        f_data.append(rr)
    max_y = len(f_data)
    max_x = len(f_data[0])

    open_costs = {(0, 0): {"weight": 0, "last": None}}
    visited_nodes = set()
    pq = queue.PriorityQueue()
    pq.put((0, (0, 0)))
    while pq.qsize() > 0:
        cw, ck = pq.get()
        if ck not in visited_nodes:
            visited_nodes.add(ck)
            for (j, i) in zip([0, 0, -1, 1], [1, -1, 0, 0]):
                cy, cx = ck
                ny = cy + j
                nx = cx + i
                nk = (ny, nx)
                if not (ny < 0 or ny >= max_y or nx < 0 or nx >= max_x):
                    nw = cw + f_data[ny][nx]
                    if nk in open_costs:
                        cc = open_costs[nk]
                        if nw < cc["weight"]:
                            open_costs[nk] = {"weight": nw, "last": ck}
                            pq.put((nw, nk))
                    else:
                        open_costs[nk] = {"weight": nw, "last": ck}
                        pq.put((nw, nk))
    return open_costs[(max_y - 1, max_x - 1)]


if __name__ == "__main__":
    with open("data.txt", "r") as f:
        data = f.read()
    sol = solve_challenge(data)
    print(sol)
