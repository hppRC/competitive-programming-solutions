N, Q = list(map(int, input().strip().split()))
S = input()
rangeList = [list(map(int, input().strip().split())) for i in range(Q)]

Accumulation = [0]*N
flag = False

if (S[0] == "A"):
    flag = True

for i in range(1,N):
    if (flag):
        if (S[i] == "C"):
            Accumulation[i] = Accumulation[i-1] + 1
            flag = False
            continue
        elif (S[i] == "A"):
            Accumulation[i] = Accumulation[i-1]
            continue
        else:
            flag = False
            Accumulation[i] = Accumulation[i-1]
            continue
    elif (S[i] == "A"):
        flag = True
    Accumulation[i] = Accumulation[i-1]

for eachRange in rangeList:
    print(Accumulation[eachRange[1]-1] - Accumulation[eachRange[0]-1])
