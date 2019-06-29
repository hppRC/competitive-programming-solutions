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

N, M = LI()
uv = LIR(M)
S, T = LI()
S, T = S-1, T-1

edge = [[0]*N for i in range(N)]
utov = [[] for i in range(N)]

for u, v in uv:
    edge[u-1][v-1] = 1
    utov[u-1].append(v-1)

kenken = []

def bfs(s):
    ret = []
    bfs_map = [-1]*N
    q = deque()
    q.append(s)
    bfs_map[s] = 0
    while q:
        u = q.popleft()
        for v in utov[u]:
            if (edge[u][v] > 0) and (bfs_map[u] > bfs_map[v]):
                bfs_map[v] = bfs_map[u] + 1
                if (bfs_map[v] == 3):
                    ret.append(v)
                else:
                    q.append(v)
    return ret

for i in range(N):
    ret = bfs(i)
    kenken.append(ret)

kenkenedge = [[0]*N for i in range(N)]
for u, vs in enumerate(kenken):
    for v in vs:
        kenkenedge[u][v] = 1

def bfs_dist(S, T):
    bfs_map = [-1]*N
    q = deque()
    q.append(S)
    bfs_map[S] = 0
    while q:
        u = q.popleft()
        for v in kenken[u]:
            if (kenkenedge[u][v] > 0) and (bfs_map[u] > bfs_map[v]):
                bfs_map[v] = bfs_map[u] + 1
                if (v==T):
                    return bfs_map[T]
                else:
                    q.append(v)
    return -1

print(bfs_dist(S, T))