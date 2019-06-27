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

def bfs(c,sy=0, sx=0, gy=0, gx=0, blocks=['#'], dire=[(1,0), (0,1), (-1,0), (0,-1)], return_maximum_distance=False):
    H = len(c)
    W = len(c[0])
    distance_limit = H*W+1
    dist = [[distance_limit]*W for i in range(H)]

    q = deque()

    q_append = q.append
    q_popleft = q.popleft

    if (type(sy)==list) or (type(sx)==list):
        for syi, sxi in zip(sy, sx):
            dist[syi][sxi] = 0
            q.append((syi, sxi))
    else:
        dist[sy][sx] = 0
        q_append((sy, sx))

    while q:
        y, x = q_popleft()
        d = dist[y][x]
        for dy, dx in dire:
            if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                if (c[y+dy][x+dx] in blocks):
                    continue
                else:
                    if (y+dy==gy) and (x+dx==gx) and (not return_maximum_distance):
                        return (d+1)
                    elif (dist[y+dy][x+dx] > d+1):
                        dist[y+dy][x+dx] = d+1
                        q_append((y+dy, x+dx))
    if (return_maximum_distance):
        return d
    return -1

H, W = LI()
c = SR(H)

sy = []
sx = []

for i in range(H):
    for j in range(W):
        if (c[i][j]=="#"):
            sy.append(i)
            sx.append(j)

print(bfs(c, sy, sx, return_maximum_distance=True))