import sys

def loadBatch(filename):

    docs = []

    current = {}

    with open(filename, "r") as f:

        for line in f:
            if len(line) == 1:
                docs.append(current)
                current = {}
                continue
            
            line = line.strip()
            words = line.split()
            for w in words:
                key, val = w.split(':')
                current[key] = val

    if len(current) > 0:
        docs.append(current)

    return docs

def checkYear(yrstr, lo, hi):

    if len(yrstr) != 4:
        return False

    try:
        yr = int(yrstr)
        if yr < lo or yr > hi:
            return False
    except ValueError:
        return False

    return True

def checkHeight(hgtStr, locm, hicm, loin, hiin):

    if not hgtStr.isalnum() or not hgtStr.islower() or len(hgtStr) < 3:
        return False

    u = hgtStr[-2:]
    try:
        hgt = int(hgtStr[:-2])
    except ValueError:
        return False

    if u not in ['cm', 'in']:
        return False

    if u == 'cm' and (hgt < locm or hgt > hicm):
        return False
    
    if u == 'in' and (hgt < loin or hgt > hiin):
        return False

    return True

def checkHexColor(colStr):

    if len(colStr) != 7:
        return False

    if colStr[0] != '#':
        return False

    if not colStr[1:].isalnum() or (colStr[1:].lower() != colStr[1:]):
        return False
    
    try:
        col = int(colStr[1:], 16)
    except ValueError:
        return False

    return True

def checkEyeColor(colStr):

    if colStr not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
        return False

    return True

def checkPID(pidStr):

    if len(pidStr) != 9:
        return False

    if not pidStr.isnumeric():
        return False

    return True


def checkBatch(docs):

    count = 0

    reqKeys = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    optKeys = {'cid'}
    valKeys = reqKeys | optKeys

    print("Checking")

    for i, doc in enumerate(docs):

        keys = set(doc.keys())

        missingReq = reqKeys - keys
        unknown = keys - valKeys

        if len(unknown) > 0:
            print("huh?", unknown, doc)

        bad = False

        if len(missingReq) > 0:
            print(i, "INVALID", "missing field:",  missingReq, doc)
            continue
       
        # BYR
        if not checkYear(doc['byr'], 1920, 2002):
            print(i, "INVALID", "bad byr:",  doc['byr'], doc)
            bad = True

        # IYR
        if not checkYear(doc['iyr'], 2010, 2020):
            print(i, "INVALID", "bad iyr:",  doc['iyr'], doc)
            bad = True
        
        # EYR
        if not checkYear(doc['eyr'], 2020, 2030):
            print(i, "INVALID", "bad eyr:",  doc['eyr'], doc)
            bad = True
        
        # HGT
        if not checkHeight(doc['hgt'], 150, 193, 59, 76):
            print(i, "INVALID", "bad hgt:",  doc['hgt'], doc)
            bad = True
        
        # HCL
        if not checkHexColor(doc['hcl']):
            print(i, "INVALID", "bad hcl:",  doc['hcl'], doc)
            bad = True
        
        # ECL
        if not checkEyeColor(doc['ecl']):
            print(i, "INVALID", "bad ecl:",  doc['ecl'], doc)
            bad = True
        
        # PID
        if not checkPID(doc['pid']):
            print(i, "INVALID", "bad pid:",  doc['pid'], doc)
            bad = True

        if not bad:
            print("VALID", doc)
            count += 1

    return count

if __name__ == "__main__":

    docs = loadBatch(sys.argv[1])

    for doc in docs:
        print(doc)
    valid = checkBatch(docs)
    print(valid)
