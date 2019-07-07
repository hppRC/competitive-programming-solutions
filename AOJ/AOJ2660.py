N = int(input())
a = list(map(int, input().split()))

def calc(a, b):
    c = 0
    ret = 0
    while (a or b or c):
        x = a%10
        y = b%10
        ret += x*y + c
        c = (x + y + c >= 10)
        a //= 10
        b //= 10
    return ret

"""
    digita = list(map(int, str(a)))
    digitb = list(map(int, str(b)))
    if (len(digita) > len(digitb)):
        tmp = [0]*len(digita)
        for i in range(len(digitb)):
            tmp[i] = digitb[i]
        digitb = tmp
    elif (len(digitb) > len(digita)):
        tmp = [0]*len(digitb)
        for i in range(len(digita)):
            tmp[i] = digita[i]
        digita = tmp
    ans = 0       
    c = 0
    for i in range(len(digita)):
        ans += digita[i]*digitb[i] + c
        c = 0 if (digita[i] + digitb[i] + c < 10) else 1
    return ans+c
"""

INF = 10000000007
dp = [[INF]*N for i in range(N)]

acc = [0]*(N+1)
for i in range(N):
    acc[i+1] = acc[i] + a[i]

def query(l, r):
    return acc[r] - acc[l]

def solve(l, r, dp):
    if (l==r):
        dp[l][r] = 0
        return
    elif (dp[l][r] != INF):
        return

    for c in range(l,r):
        solve(l, c, dp)
        solve(c+1, r, dp)
        dp[l][r] = min( dp[l][r], dp[l][c] + dp[c+1][r] + calc(query(l, c +1), query(c+1, r +1)) )



"""
    solve(l, r-1, dp)
    solve(r, r, dp)
    dp[l][r] = min(dp[l][r], dp[l][r-1] + dp[r][r] + calc(query(l, r), query(r, r+1)))
    solve(l, l, dp)
    solve(l+1, r, dp)
    dp[l][r] = min(dp[l][r], dp[l][l] + dp[l+1][r] + calc(query(l, l+1), query(l+1, r+1)))


        print(l, c, r)
        tmp = solve(l, c, dp) + solve(c+1, r, dp) + calc(query(l, c), query(c+1, r+1))

        dp[l][r] = min(dp[l][r], tmp)

    for i in dp:
        print(i)
    return dp[l][r]
"""    

solve(0, N-1, dp)
print(dp[0][N-1])

"""
def solve(l, r, dp):
    if (dp[l][r]):
        return dp[l][r]
    if (r-l <= 1):
        return 0
    t1 = acc[r-1] - acc[l]
    t2 = acc[r] - acc[l+1]

    tmp1 = solve(l, r-1, dp) + calc(t1, a[r-1])
    tmp2 = solve(l+1, r, dp) + calc(t2, a[l])
    
    dp[l][r] = min(tmp1, tmp2)

    return dp[l][r]

for l in range(N):
    for r in range(l,N):
        dp[l][r+1] = min(dp[l][r+1], dp[l][r] + calc(acc[r]-acc[l], a[r]))
        dp[l-1][r] = min(dp[l-1][r], dp[l][r] + calc(acc[r]-acc[l], a[l-1]))


print(solve(0, N, dp))
"""