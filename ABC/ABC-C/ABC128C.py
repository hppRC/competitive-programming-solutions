N, M = map(int, input().split())
KS = [list(map(int, input().split())) for i in range(M)]
P = list(map(int, input().split()))
ans = 0
for i in range(1<<N):
    flag = True
    for j in range(M):
        k = KS[j][0]
        s = sorted(list(map(lambda x: x-1, KS[j][1:])))
        p = P[j]
        count = 0
        comb = 0
        for si in s:
            comb = comb | (1<<si)
        res = i & comb
        for l in range(N):
            if (res>>l)&1:
                count += 1
        if (count%2 == p):
            continue
        else:
            flag = False
    if (flag):
        ans += 1


print(ans)