import sys
import numpy as np

if __name__ == "__main__":

    jolts = np.loadtxt(sys.argv[1], dtype=int)
    jolts.sort()

    jolts = np.concatenate([[0], jolts, [jolts.max()+3]])

    diffs = jolts[1:] - jolts[:-1]

    if (diffs == 0).any():
        print("Copy!")
    if (diffs > 3).any():
        print("Can't use all!")

    ones = np.ones(diffs.shape, dtype=int)

    n1 = ones[diffs == 1].sum()
    n2 = ones[diffs == 2].sum()
    n3 = ones[diffs == 3].sum()

    print("Number 1 diffs:", n1)
    print("Number 2 diffs:", n2)
    print("Number 3 diffs:", n3)
    print(n1 * n3)

