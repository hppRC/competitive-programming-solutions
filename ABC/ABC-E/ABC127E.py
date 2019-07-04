N, M, K = map(int, input().split())

dp = [[[0 for k in range(K+1)] for j in range(M+1)] for i in range(N+1)]


for i in range(N):
    for j in range(M):
        dp[i][j][1] = 1


for i in range(N):
    for j in range(M):
        for k in range(1, K):
            dp[i][j][k+1] = (i*j-k)*dp[i][j][k]

for i in dp:
    print(i)

print(dp[N][M][K])
