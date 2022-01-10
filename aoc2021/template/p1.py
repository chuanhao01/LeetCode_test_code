import typing as t


def solve(data: str) -> t.Any:
    pass


def read_data(data_path: str = "data.txt") -> str:
    data = None
    with open(data_path, "r", encoding="utf-8") as f:
        data = f.read()
    return data


if __name__ == "__main__":
    data = read_data()
    solution = solve(data)
    print(solution)
