#!usr/bin/env python3
from collections import defaultdict
from collections import deque
from heapq import heappush, heappop
import sys
import math
import bisect
import random
def LI(): return list(map(int, sys.stdin.readline().split()))
def I(): return int(sys.stdin.readline())
def LS():return list(map(list, sys.stdin.readline().split()))
def S(): return list(sys.stdin.readline())[:-1]
def IR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = I()
    return l
def LIR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = LI()
    return l
def SR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = S()
    return l
def LSR(n):
    l = [None for i in range(n)]
    for i in range(n):l[i] = LS()
    return l
sys.setrecursionlimit(1000000)
mod = 1000000007
dire4 = [(1,0), (0,1), (-1,0), (0,-1)]
dire8 = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

N = I()
abc = LIR(N-1)
Q, K = LI()
xy = LIR(Q)


K = K

utov = [[] for i in range(N+1)]
for a, b, c in abc:
    utov[a].append((b,c))
    utov[b].append((a,c))

bfs_map = [-1]*(N+1)

q = deque()
q.append(K)
bfs_map[K] = 0

while q:
    u = q.popleft()
    for v, cost in utov[u]:
        if (bfs_map[v] == -1) or (bfs_map[v] > bfs_map[u]+cost):
            bfs_map[v] = bfs_map[u] + cost
            q.append(v)


for x, y in xy:
    print(bfs_map[x]+bfs_map[y])
