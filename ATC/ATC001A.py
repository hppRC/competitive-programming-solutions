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
field = [[0]*W for i in range(H)]

dire = [(1,0), (0,1), (-1,0), (0,-1)]

q = deque()

for i in range(H):
    for j in range(W):
        if c[i][j] == "s":
            q.append((i, j))
            break
    else:
        continue
    break

while q:
    v = q.popleft()
    field[v[0]][v[1]] = 1
    for dy, dx in dire:
        if (v[0]+dy<H and v[0]+dy>=0 and v[1]+dx<W and v[1]+dx>=0):
            if (c[v[0]+dy][v[1]+dx] == ".") and (not field[v[0]+dy][v[1]+dx]):
                q.append((v[0]+dy, v[1]+dx))
                field[v[0]+dy][v[1]+dx] = 1
            elif (c[v[0]+dy][v[1]+dx] == "g"):
                print("Yes")
                quit()

print("No")