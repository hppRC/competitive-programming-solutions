def root(x):
    if par[x] == x:
        return x
    else:
        par[x] = root(par[x])
        return par[x]

def same(x, y):
    return root(x) == root(y)

def unite(x, y):
    x = root(x)
    y = root(y)
    if rank[x] == rank[y]:
        par[y] = x
        rank[x] += 1
    elif rank[x] > rank[y]:
        par[y] = x
    elif rank[x] < rank[y]:
        par[x] = y
    
N, M = map(int, input().split())
par = [i for i in range(N)]
rank = [0 for i in range(N)]

for i in range(M):
    x, y, z = map(int, input().split())
    x -= 1
    y -= 1
    if not same(x, y):
        unite(x, y)
    
for i in range(N):
    root(i)

par = list(set(par))
print(len(par))
