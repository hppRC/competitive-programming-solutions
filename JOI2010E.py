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

H, W, N = LI()
c = SR(H)

for i in range(H):
    for j in range(W):
        if (c[i][j] == "S"):
            sy, sx = i, j

dist = [[mod]*W for i in range(H)]
dist[sy][sx] = 0
strength = 1

dire = [(1, 0), (0, 1), (-1, 0), (0, -1)]

q = deque()
q.append((sy, sx))

while q:
    y, x = q.popleft()
    for dy, dx in dire:
        if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
            if (c[y+dy][x+dx]=="X"):
                continue
            elif ((c[y+dy][x+dx]==".") or (c[y+dy][x+dx]=="S")):
                if  (dist[y+dy][x+dx] > dist[y][x]+1):
                    dist[y+dy][x+dx] = dist[y][x]+1
                    q.append((y+dy, x+dx))
            else:
                cheese = int(c[y+dy][x+dx])
                if (strength == cheese):
                    strength += 1
                    time = dist[y][x]+1

                    if (strength == N+1):
                        print(time)
                        quit()

                    q = deque()
                    q.append((y+dy, x+dx))
                    dist = [[mod]*W for i in range(H)]
                    dist[y+dy][x+dx] = time
                    break
                else:
                    if  (dist[y+dy][x+dx] > dist[y][x]+1):
                        dist[y+dy][x+dx] = dist[y][x]+1
                        q.append((y+dy, x+dx))





