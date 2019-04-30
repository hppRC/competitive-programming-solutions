N, M = list(map(int, input().split()))
li = [list(map(int, input().split()))[1:] for i in range(N)]

Ms = list(range(1, M+1))
ans = list(range(1, M+1))

for i in li:
    ans = list(set(ans) - (set(Ms) - set(i)))

print(len(ans))

