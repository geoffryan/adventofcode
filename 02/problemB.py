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

        n = len(pswd)

        occs = 0

        if a-1 < n:
            if pswd[a-1] == ch:
                occs += 1
        if b-1 < n:
            if pswd[b-1] == ch:
                occs += 1

        if occs == 1:
            count += 1

print(count)


