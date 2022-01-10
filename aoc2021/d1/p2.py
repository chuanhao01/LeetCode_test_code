import typing as t


def solve(data: str) -> t.Any:
    depths = [int(depth) for depth in data.split("\n")]
    count = 0
    for i in range(3, len(depths)):
        count += (
            1
            if depths[i] + depths[i - 1] + depths[i - 2]
            > depths[i - 1] + depths[i - 2] + depths[i - 3]
            else 0
        )
    return count


def read_data(data_path: str = "data.txt") -> str:
    data = None
    with open(data_path, "r", encoding="utf-8") as f:
        data = f.read()
    return data


if __name__ == "__main__":
    data = read_data()
    solution = solve(data)
    print(solution)
