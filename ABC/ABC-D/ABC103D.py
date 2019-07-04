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
ab = LIR(M)

ab = sorted(ab, key=lambda x: (x[1], x[0]))

before = 0
ans = 0
for ai, bi in ab:
    if (ai <= before):
        continue
    else:
        ans += 1
        before = bi-1
print(ans)

"""
left = [0]*N
right = [0]*N

for ai, bi in ab:
    left[ai-1] = 1
    right[bi-1] = 1

l = N-1
r = N-1
ans = 0

flag = False
while (l>=0):
    if (left[l]):
        while (r>l):
            if (right[r]):
                flag = True
            r -= 1
        if (flag):
            ans += 1
            flag = False
    l -= 1

print(ans)
"""