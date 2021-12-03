import sys

total = 0

with open(sys.argv[1], "r") as f:

    answers = set('abcdefghijklmnopqrstuvwxyz')

    for line in f:
        line = line.strip()
        if len(line) <= 0:
            print(len(answers))
            total += len(answers)
            answers = set('abcdefghijklmnopqrstuvwxyz')
            continue
        answers = answers & set(line)

    print(len(answers))
    total += len(answers)

print(total)
