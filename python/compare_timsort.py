import matplotlib.pyplot as plt

def get_xs_and_ys(filename):
    file = open(filename)

    xs = []
    ys = []

    for line in file.read().splitlines():
        [x, y] = line.split()
        xs.append(int(x))
        ys.append(float(y))

    return [xs, ys]

def get_xs_and_ys_by_n(filename):
    file = open(filename)

    xs = []
    ys = []

    for line in file.read().splitlines():
        [x, y] = line.split()
        x = int(x)
        xs.append(int(x))
        ys.append(float(y) / x)

    return [xs, ys]

def plot_comps():
    [merge_xs, merge_ys] = get_xs_and_ys("merge_comps")
    [tim_xs, tim_ys] = get_xs_and_ys("timsort_comps")

    _, ax = plt.subplots()

    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(tim_xs, tim_ys, label = "timsort")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("comparisons")

    plt.savefig("comparisons", dpi=500)

def plot_swaps():
    [merge_xs, merge_ys] = get_xs_and_ys("merge_swaps")
    [tim_xs, tim_ys] = get_xs_and_ys("timsort_swaps")

    _, ax = plt.subplots()

    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(tim_xs, tim_ys, label = "timsort")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("swaps")

    plt.savefig("swaps", dpi=500)

def main():
    plot_comps()
    plot_swaps()

if __name__ == "__main__":
    main()
