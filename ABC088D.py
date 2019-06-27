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

H, W = LI()
c = SR(H)
sy, sx = 0, 0
gy, gx = H-1, W-1

white_counts = 0
for i in range(H):
    for j in range(W):
        if (c[i][j]=="."):
            white_counts += 1

def bfs(sy, sx, gy, gx, c, blocks=['#']):
    H = len(c)
    W = len(c[0])
    dire = [(1,0), (0,1), (-1,0), (0,-1)]
    dist = [[1000000007]*W for i in range(H)]
    dist[sy][sx] = 0
    q = deque()
    q.append((sy, sx))
    while q:
        y, x = q.popleft()
        for dy, dx in dire:
            if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                if (c[y+dy][x+dx] in blocks):
                    continue
                else:
                    if (y+dy==gy) and (x+dx==gx):
                        return (dist[y][x]+1)
                    elif (dist[y+dy][x+dx] > dist[y][x]+1):
                        dist[y+dy][x+dx] = dist[y][x]+1
                        q.append((y+dy, x+dx))
    return -1

path = bfs(sy, sx, gy, gx, c)
print(white_counts-path-1 if (path!=-1) else -1)

