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

def checkBatch(docs):

    count = 0

    reqKeys = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    optKeys = {'cid'}
    valKeys = reqKeys | optKeys

    print("Checking")

    for doc in docs:

        keys = set(doc.keys())

        missingReq = reqKeys - keys
        unknown = keys - valKeys

        if len(unknown) > 0:
            print("huh?", unknown, doc)

        if len(missingReq) == 0:
            count += 1
            print("VALID", doc)
        else:
            print("INVALID", missingReq, doc)

    return count

if __name__ == "__main__":

    docs = loadBatch(sys.argv[1])

    for doc in docs:
        print(doc)
    valid = checkBatch(docs)
    print(valid)
