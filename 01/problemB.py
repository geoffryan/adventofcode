import numpy as np


def search(vals, target):

    N = len(vals)
    a = 0
    b = N-1

    if vals[0] + vals[1] > target:
        return None

    if vals[-1] + vals[-2] < target:
        return None

    while True:
        s = vals[a] + vals[b]
        if s == target:
            return vals[a], vals[b]
        elif s > target:
            b -= 1
        else:
            a += 1

        if a == b:
            break

    return None

if __name__ == "__main__":

    vals = np.loadtxt('input.txt', dtype=int, unpack=True)
    vals.sort()

    N = len(vals)

    for i in range(N):

        if i == 0:
            copy = vals[1:]
        elif i == N-1:
            copy = vals[:-1]
        else:
            copy = np.concatenate((vals[:i], vals[i+1:]))

        res = search(vals, 2020 - vals[i])

        if res is not None:
            print(res[0] * res[1] * vals[i])
            break





