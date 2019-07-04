N, M = map(int, input().split())
A = sorted(list(map(int, input().split())))
BC = sorted([list(map(int, input().split())) for i in range(M)], key=lambda x: x[1])
index = N-1
ans = 0
limit = 0

for j in range(M-1, -1, -1):
    Bj = BC[j][0]
    Cj = BC[j][1]
    while True:
        if (A[index]<Cj):
            break
        else:
            ans += A[index]
            index -= 1
            if (index < 0):
                break
    if (Bj > index - limit + 1):
        ans += Cj*(index - limit + 1)
        limit = index + 1
        break
    ans += Bj*Cj
    limit += Bj
    
ans += sum(A[limit:index+1])

print(ans)
