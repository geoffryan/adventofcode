import sys
import numpy as np

mask = np.zeros(8 * 128, dtype=np.bool)

with open(sys.argv[1], "r") as f:
    for line in f:
        line = line.strip()
        bitString = line.replace("F", "0").replace("B", "1")\
                        .replace("L", "0").replace("R", "1")
        seatID = int(bitString, 2)
        mask[seatID] = True

print( np.argwhere((~mask[1:-1]) & (mask[2:]) & (mask[:-2])) + 1)
