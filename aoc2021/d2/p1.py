import typing as t


def solve(data: t.Any) -> t.Any:
    f = 0
    d = 0
    for dd in data:
        s = dd[0]
        v = dd[1]
        if s == "forward":
            f += v
        if s == "up":
            d -= v
        if s == "down":
            d += v
    return f * d


def read_data(data_path: str = "data.txt") -> t.Any:
    with open(data_path, "r", encoding="utf-8") as f:
        data = f.read()
    parsed_data = [d.split(" ") for d in data.split("\n")]
    parsed_data = [[d[0], int(d[1])] for d in parsed_data]
    return parsed_data


if __name__ == "__main__":
    data = read_data()
    solution = solve(data)
    print(solution)
