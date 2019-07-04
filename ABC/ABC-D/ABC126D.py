from collections import deque

N = int(input())
v = [[] for i in range(N)]

for i in range(N-1):
    x, y, w = map(int, input().split())
    v[x-1].append((y-1, w))
    v[y-1].append((x-1, w))

ans = [0 for i in range(N)]
bfs_map = [1 for i in range(N)]
bfs_map[0] = 0

q = deque()
q.append(0)

while q:
    x = q.popleft()
    for y, w in v[x]:
        if bfs_map[y]:
            if w%2 :
                ans[y] = ans[x] ^ 1
            else:
                ans[y] = ans[x]
            bfs_map[y] = 0
            q.append(y)
    
for i in ans:
    print(i)