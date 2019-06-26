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

island = SR(10)

dire = [(1,0), (0,1), (-1,0), (0,-1)]

for i in range(10):
    for j in range(10):
        before = island[i][j]
        island[i][j] = "o"
        q = deque()
        q.append((i, j))

        filed = [[0]*10 for i in range(10)]

        while q:
            v = q.popleft()
            filed[v[0]][v[1]] = 1
            for dx, dy in dire:
                if (v[0]+dy<10 and v[0]+dy>=0 and v[1]+dx<10 and v[1]+dx>=0):
                    if (island[v[0]+dy][v[1]+dx] == "o") and (not filed[v[0]+dy][v[1]+dx]):
                        q.append((v[0]+dy, v[1]+dx))
                    
        for k in range(10):
            for l in range(10):
                if (island[k][l]=="o") and (not filed[k][l]):
                    break
            else:
                continue
            break
        else:
            print("YES")
            quit()
        island[i][j] = before 
        
print("NO")


