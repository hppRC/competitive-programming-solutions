N, K = list(map(int, input().split()))
h = list(map(int, input().split()))

dp = [0 for i in range(N)]
dp[1] = dp[0] + abs(h[1] - h[0])

for i in range(2, N):
    if (i-K < 0) :
        nums = list(range(i))
    else :
        nums = list(range(i-K, i))
    li = [dp[num] + abs(h[i] - h[num]) for num in nums]
    dp[i] = min(li)

print(dp[N-1])