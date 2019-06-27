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

R, C = LI()
sy, sx = LI()
gy, gx = LI()

sy, sx = (sy-1, sx-1)
gy, gx = (gy-1, gx-1)

c = SR(R)

dist = [[mod]*C for i in range(R)]

q = deque()
q.append((sy, sx))
dist[sy][sx] = 0

dire = [(1,0), (0, 1), (-1, 0), (0, -1)]

while q:
    y, x = q.popleft()
    for dy, dx in dire:
        if (y+dy<R) and (y+dy>=0) and (x+dx<C) and (x+dx>=0):
            if (c[y+dy][x+dx]==".") and (dist[y+dy][x+dx] > dist[y][x]+1):
                q.append((y+dy, x+dx))
                dist[y+dy][x+dx] = dist[y][x]+1

print(dist[gy][gx])
