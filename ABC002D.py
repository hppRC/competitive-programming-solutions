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

N, M = LI()
xy = list(map(lambda x: [x[0]-1, x[1]-1], LIR(M)))

xtoy = [[] for i in range(N)]

ans = 1

for xi, yi in xy:
    xtoy[xi].append(yi)

for i in range(1, 1<<N):
    bit = i
    group = []

    for j in range(N):
        if (bit & 1):
            group.append(j)
        bit >>= 1

    count = 0
    for member in group:
        for person in xtoy[member]:
            if (person in group):
                count += 1
    
    if (count == (len(group)*(len(group)-1)//2)):
        ans = max(ans, len(group))

print(ans)