N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

def donguri(n, a, b):
    dp = [0]*(n+1)
    for w in range(n+1):
        x0 = dp[w-a[0]] + b[0] if (w-a[0]>=0) else w
        x1 = dp[w-a[1]] + b[1] if (w-a[1]>=0) else w
        x2 = dp[w-a[2]] + b[2] if (w-a[2]>=0) else w
        dp[w] = max(w, x0, x1, x2)
    return dp[-1]

print(donguri(donguri(N,A,B),B,A))
