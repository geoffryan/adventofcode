import sys

seat = 0

with open(sys.argv[1], "r") as f:
    for line in f:
        line = line.strip()
        bitString = line.replace("F", "0").replace("B", "1")\
                        .replace("L", "0").replace("R", "1")
        seatID = int(bitString, 2)
        print(seatID)
        seat = max(seat, seatID)

print("max:", seat)
