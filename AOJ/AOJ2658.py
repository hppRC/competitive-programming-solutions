from collections import deque
import bisect

INF = float("inf")

N = int(input())
C = list(map(int, input().split()))
uvp = [list(map(int, input().split())) for i in range(N-1)]

edge = [[0]*(N+1) for i in range(N+1)]
utov = [[] for i in range(N+1)]

for u, v, p in uvp:
    edge[u][v] = edge[v][u] = p
    utov[u].append(v)
    utov[v].append(u)

for i in range(1, N):
    if (C[i-1] > 0):
        edge[i][N] = INF
        utov[i].append(N)
        utov[N].append(i)


def bfs(s, g, n):
    bfs_map = [-1]*n
    bfs_map[s] = 0
    q = deque()
    q.append(s)

    while q:
        u = q.popleft()
        for v in utov[u]:
            if (edge[u][v] > 0) and (bfs_map[v] < 0):
                bfs_map[v] = bfs_map[u] + 1
                q.append(v)

    return bfs_map

def update(s, g, bfs_map):
    f = INF
    distance = bfs_map[g]
    p = [[None, None] for i in range(distance)]
    y = g
    for i in range(distance)[::-1]:
        p[i][1] = y
        for x in utov[y]:
            if (edge[x][y] > 0) and (bfs_map[x] == i):
                if (edge[x][y] < f):
                    f = edge[x][y]
                y = x
                p[i][0] = x
                break
        else:
            return 0
    for x,y in p:
        edge[x][y] -= f
        edge[y][x] += f
    return f

def dinic(s, g, n):
    max_flow = 0
    while True:
        bfs_map = bfs(s, g, n)
        if (bfs_map[g] < 0):
            return max_flow
        cap = update(s, g, bfs_map)
        while (cap > 0):
            max_flow += cap
            cap = update(s, g, bfs_map)

print(dinic(0, N, N+1))




"""
def update(s, g, dist):



utov = [[] for i in range(N+1)]
edge = [[0]*(N+1) for i in range(N+1)]

for u, v, p in uvp:
    edge[u][v] = edge[v][u] = p
    utov[u].append(v)
    utov[v].append(u)

dist = [-1]*N
dist[0] = 0

q = deque()
q.append(0)

while q:
    u = q.popleft()
    for v in utov[u]:
        if (edge[u][v]) and (dist[v] < 0):
            dist[v] = dist[u] + 1
            q.append(v)

print(dist)





cost = [0]*N
q = deque()
q.append(0)
while q:
    u = q.popleft()
    for v in utov[u]:
        if (not cost[v]):
            cost[u] += edge[u][v]
            q.append(v)

ans = 0
for i, c in enumerate(C):
    if (not c):
        q.append(i)
    while q:



print(cost)
print(C)
"""