count = 0
slope = 3
pos = 0
width = 0

with open("input.txt", "r") as f:

    for i, line in enumerate(f):
        line = line.strip()
        if i == 0:
            width = len(line)
            print(width)
        else:
            pos += slope
            while pos >= width:
                pos -= width

            # print(line)

            if line[pos] == '#':
                count += 1
                newline = line[:pos] + 'X' + line[pos+1:]
            else:
                newline = line[:pos] + 'O' + line[pos+1:]

            print(newline, count, pos)


print(count)
