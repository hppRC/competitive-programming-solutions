N = int(input())
SP = [input().split()+[i+1] for i in range(N)]
SP = sorted(SP, key=lambda x: (x[0], -int(x[1])))

for i in SP:
    print(i[2])