import sys

if __name__ == "__main__":

    filename = sys.argv[1]

    with open(filename, "r") as f:

        prog = []
        for line in f:
            prog.append(line.strip().split())

    ist = 0
    cnt = 0
    acc = 0

    visited = []

    n = len(prog)

    while True:
        cmd = prog[ist][0]
        arg = prog[ist][1]
        
        if ist in visited:
            print("LOOP")
            break
        if ist >= n:
            print("END")
            break
        
        visited.append(ist)
        
        print(cnt, ist, cmd, arg, "|", acc)

        if cmd == "nop":
            ist += 1
        elif cmd == "acc":
            acc += int(arg)
            ist += 1
        else:
            ist += int(arg)
            
        cnt += 1

    print(acc)
