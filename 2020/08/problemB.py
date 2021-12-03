import sys

if __name__ == "__main__":

    filename = sys.argv[1]

    with open(filename, "r") as f:

        prog = []
        for line in f:
            prog.append(line.strip().split())

    inst = 0
    acc = 0
    cnt = 0

    visited = []

    n = len(prog)

    breakpoint = None
    badRun = None

    while True:
        if inst >= n:
            print("END")
            break
        
        cmd = prog[inst][0]
        arg = prog[inst][1]
        
        if inst in visited:
            print("LOOP - ", "counter:", cnt, "instruction:", inst)

            if breakpoint is None:
                badRun = visited.copy()
                breakpoint = cnt-1
            else:
                if prog[badRun[breakpoint]][0] == 'nop':
                    prog[badRun[breakpoint]][0] = 'jmp'
                else:
                    prog[badRun[breakpoint]][0] = 'nop'
                breakpoint -= 1

            while prog[badRun[breakpoint]][0] == 'acc':
                breakpoint -= 1

            print("New switch point:", breakpoint, "instruction",
                  badRun[breakpoint])
            
            if prog[badRun[breakpoint]][0] == 'nop':
                prog[badRun[breakpoint]][0] = 'jmp'
            else:
                prog[badRun[breakpoint]][0] = 'nop'

            cnt = 0
            inst = 0
            acc = 0
            visited = []
            continue
        
        if cnt < len(visited):
            vistited[cnt] = inst
        else:
            visited.append(inst)
        
        if cmd == "nop":
            inst += 1
        elif cmd == "acc":
            acc += int(arg)
            inst += 1
        else:
            inst += int(arg)

        cnt += 1

    print(acc)
