count = 0

with open('input.txt', 'r') as f:
    for line in f:

        tok = line.split()
        num = tok[0]
        ch = tok[1][:-1]
        pswd = tok[2]

        numab = num.split('-')
        a = int(numab[0])
        b = int(numab[1])

        occ = pswd.count(ch)

        if len(ch) > 1:
            print("whoa")
        if len(ch) < 1:
            print("huh")

        # print(a, b, ch, pswd, occ)

        if occ >= a and occ <= b:
            count += 1


print(count)


