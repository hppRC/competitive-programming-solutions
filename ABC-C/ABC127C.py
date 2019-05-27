N, M = map(int, input().split())
LR = [list(map(int, input().split())) for i in range(M)]
imos = [0 for i in range(N+2)]
ans = 0
for lr in LR:
    imos[lr[0]] += 1
    imos[lr[1]+1] -= 1

for i in range(1, N+2):
    imos[i] += imos[i-1]
    ans += 1 if (imos[i]==M) else 0

print(ans)