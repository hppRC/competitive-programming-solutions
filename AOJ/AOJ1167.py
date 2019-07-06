"""
tetrahedral = [i*(i+1)*(i+2)//6 for i in range(1,185)]
oddtetrahedral = [i*(i+1)*(i+2)//6 for i in range(1,185,4)]
INF = float("inf")
print(oddtetrahedral)"""
dp = [1e6+1]*(int(1e6)+1)
dpodd = [1e6+1]*(int(1e6)+1)

dp[0] = dpodd[0] = 0

for i in range(1, 181):
    num = i*(i+1)*(i+2)//6
    for j, tpl in enumerate(zip(dp[num:num*3+1], dp), start=num):
        a, b = tpl
        if (b+1 < a):
            dp[j] = b+1
    if (num&1):
        for j, tpl in enumerate(zip(dpodd[num:num*34+1], dpodd), start=num):
            a, b = tpl
            if (b+1 < a):
                dpodd[j] = b+1


"""
for i in range(int(1e6)):
    num = i*(i+1)*(i+2)//6
    if (num>=1e6):
        break
    for j in range(num, int(1e6)):
        dp[j] = min(dp[j], dp[j-num]+1)
        if (num&1):
            dpodd[j] = min(dpodd[j], dpodd[j-num]+1)


dp = [INF]*int(1e6+1)
dpodd = [INF]*int(1e6+1)

for a in tetrahedral:
    if (a > int(1e6)):
        break
    dp[a] = 1
for b in oddtetrahedral:
    if (b > int(1e6)):
        break
    dpodd[b] = 1

for i in range(int(1e6)+1):
    for a in tetrahedral:
        if (i+a <= int(1e6)):
            dp[i+a] = min(dp[i+a], dp[i]+1)
    for b in oddtetrahedral:
        if (i+b <= int(1e6)):
            dpodd[i+b] = min(dpodd[i+b], dpodd[i]+1)
"""

def solve(N):
    return (dp[N], dpodd[N])

"""
    dp = [INF]*(N+1)
    dpodd = [INF]*(N+1)
    for a in tetrahedral:
        if (a > N):
            break
        dp[a] = 1
    for b in oddtetrahedral:
        if (b > N):
            break
        dpodd[b] = 1

    def reca(n):
        if (n < 0):
            return INF
        if (dp[n] != INF):
            return dp[n]
        dp[n] = min([reca(n-a)+1 for a in tetrahedral if (n-a>=0)])
        return dp[n]

    def recb(n):
        if (n < 0):
            return INF
        if (dpodd[n] != INF):
            return dpodd[n]
        dpodd[n] = min([recb(n-b)+1 for b in oddtetrahedral])
        return dpodd[n]

    ret1 = reca(N)
    ret2 = recb(N)
    return (ret1, ret2)
"""

def main():
    ans = []
    while True:
        N = int(input())
        if (N == 0):
            break
        ans.append(solve(N))

    for i in ans:
        print(*i)

main()


"""
    for i in range(N+1):
        for a in tetrahedral:
            if (i+a <= N):
                dp[i+a] = min(dp[i+a], dp[i]+1)
        for b in oddtetrahedral:
            if (i+b <= N):
                dpodd[i+b] = min(dpodd[i+b], dpodd[i]+1)
    return (dp[N], dpodd[N])
"""