import sys

def loadMap(filename):

    treemap = []

    with open(filename, "r") as f:
        for line in f:
            line = line.strip()
            treemap.append(line)

    return treemap


def check(slopeX, slopeY, treeMap):

    count = 0
    pos = 0
    height = len(treeMap)
    width = len(treeMap[0])

    for j in range(slopeY, height, slopeY):

        line = treeMap[j]
        
        pos += slopeX
        while pos >= width:
            pos -= width

        # print(line)

        if line[pos] == '#':
            count += 1
            newline = line[:pos] + 'X' + line[pos+1:]
        else:
            newline = line[:pos] + 'O' + line[pos+1:]

        print(newline, count, pos)

    return count


if __name__ == "__main__":

    treemap = loadMap(sys.argv[1])

    c1 = check(1, 1, treemap)
    c2 = check(3, 1, treemap)
    c3 = check(5, 1, treemap)
    c4 = check(7, 1, treemap)
    c5 = check(1, 2, treemap)

    print(c1, c2, c3, c4, c5)
    print(c1*c2*c3*c4*c5)

