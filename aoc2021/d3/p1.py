import typing as t


def solve(data: t.Any) -> t.Any:
    gamma = ""
    epsilon = ""
    for i in range(len(data[0])):
        o = 0
        z = 0
        for d in data:
            dd = d[i]
            if dd == "0":
                z += 1
            if dd == "1":
                o += 1
        if z > o:
            gamma += "0"
            epsilon += "1"
        else:
            gamma += "1"
            epsilon += "0"
    print(gamma, epsilon)
    return int(gamma, 2) * int(epsilon, 2)


def read_data(data_path: str = "data.txt") -> t.Any:
    with open(data_path, "r", encoding="utf-8") as f:
        data = f.read()
    parsed_data = [d for d in data.split("\n")]
    return parsed_data


if __name__ == "__main__":
    data = read_data()
    solution = solve(data)
    print(solution)
