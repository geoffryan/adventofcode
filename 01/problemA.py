import numpy as np

vals = np.loadtxt('input.txt', dtype=int, unpack=True)

vals.sort()

N = len(vals)

a = 0
b = N-1

while True:
    s = vals[a] + vals[b]
    if s == 2020:
        print(vals[a] * vals[b])
        break
    elif s > 2020:
        b -= 1
    else:
        a += 1

    if a == b:
        break

