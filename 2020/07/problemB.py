import sys

def parseLine(line):

    words = line.split()

    outerColor = words[0] + " " + words[1]
    contains = {}

    if not (words[2] == "bags" and words[3] == "contain"):
        print("ERROR", line)

    if words[4] == "no" and words[5] == "other" and words[6] == "bags":
        return outerColor, contains

    N = len(words)
    for i in range((N-4)//4):
        idx = 4+4*i
        num = int(words[idx])
        color = words[idx+1] + " " + words[idx+2]
        contains[color] = {'n': num}

    return outerColor, contains

def checkColor(color, target, rules, containsTarget):

    if color in containsTarget:
        return True

    for subcolor in rules[color].keys():
        if (subcolor == target
                or checkColor(subcolor, target, rules, containsTarget)):
            containsTarget.add(color)
            return True

    return False


def countNumberContains(color, rules, totalNumber):

    if color in totalNumber:
        return totalNumber[color]

    count = 0

    for subcolor in rules[color].keys():
        num = countNumberContains(subcolor, rules, totalNumber)
        count += rules[color][subcolor]['n'] * (num + 1)

    totalNumber[color] = count

    return count


if __name__ == "__main__":

    rules = {}

    with open(sys.argv[1], 'r') as f:
        for line in f:
            line = line.strip()
            color, contains = parseLine(line)

            if color not in rules:
                rules[color] = contains
            else:
                rules[color].update(contains)

    totalNumber = {}

    count = countNumberContains("shiny gold", rules, totalNumber)

    print(totalNumber)
    print(count)
        

