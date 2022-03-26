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
    [insertion_xs, insertion_ys] = get_xs_and_ys("insertion_comps")
    [merge_xs, merge_ys] = get_xs_and_ys("merge_comps")
    [quick_xs, quick_ys] = get_xs_and_ys("quick_comps")

    _, ax = plt.subplots()

    ax.plot(insertion_xs, insertion_ys, label = "insertion")
    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("comparisons")

    plt.savefig("comparisons", dpi=500)

def plot_swaps():
    [insertion_xs, insertion_ys] = get_xs_and_ys("insertion_swaps")
    [merge_xs, merge_ys] = get_xs_and_ys("merge_swaps")
    [quick_xs, quick_ys] = get_xs_and_ys("quick_swaps")

    _, ax = plt.subplots()

    ax.plot(insertion_xs, insertion_ys, label = "insertion")
    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("swaps")

    plt.savefig("swaps", dpi=500)

def plot_comps_by_n():
    [insertion_xs, insertion_ys] = get_xs_and_ys_by_n("insertion_comps")
    [merge_xs, merge_ys] = get_xs_and_ys_by_n("merge_comps")
    [quick_xs, quick_ys] = get_xs_and_ys_by_n("quick_comps")

    _, ax = plt.subplots()

    ax.plot(insertion_xs, insertion_ys, label = "insertion")
    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("comparisons_by_n")

    plt.savefig("comparisons_by_n", dpi=500)

def plot_swaps_by_n():
    [insertion_xs, insertion_ys] = get_xs_and_ys_by_n("insertion_swaps")
    [merge_xs, merge_ys] = get_xs_and_ys_by_n("merge_swaps")
    [quick_xs, quick_ys] = get_xs_and_ys_by_n("quick_swaps")

    _, ax = plt.subplots()

    ax.plot(insertion_xs, insertion_ys, label = "insertion")
    ax.plot(merge_xs, merge_ys, label = "merge")
    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("swaps_by_n")

    plt.savefig("swaps_by_n", dpi=500)

def main():
    plot_comps()
    plot_swaps()
    plot_comps_by_n()
    plot_swaps_by_n()

if __name__ == "__main__":
    main()
