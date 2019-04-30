N, X = list(map(int, input().split()))
M = [int(input()) for i in range(N)]
print(N+int((X-sum(M))/min(M)))
