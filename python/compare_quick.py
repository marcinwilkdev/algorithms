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
    [quick_xs, quick_ys] = get_xs_and_ys("quick_comps")
    [dual_xs, dual_ys] = get_xs_and_ys("dual_pivot_comps")

    _, ax = plt.subplots()

    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.plot(dual_xs, dual_ys, label = "dual_pivot")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("comparisons")

    plt.savefig("comparisons", dpi=500)

def plot_swaps():
    [quick_xs, quick_ys] = get_xs_and_ys("quick_swaps")
    [dual_xs, dual_ys] = get_xs_and_ys("dual_pivot_swaps")

    _, ax = plt.subplots()

    ax.plot(quick_xs, quick_ys, label = "quick")
    ax.plot(dual_xs, dual_ys, label = "duapl_pivot")
    ax.legend()

    plt.xlabel("n")
    plt.ylabel("swaps")

    plt.savefig("swaps", dpi=500)

def main():
    plot_comps()
    plot_swaps()

if __name__ == "__main__":
    main()
