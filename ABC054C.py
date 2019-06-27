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

for i in range(M):
    ab[i] = [ab[i][0]-1, ab[i][1]-1]

edge = [[0]*N for i in range(N)]

for e in ab:
    edge[e[0]][e[1]] = edge[e[1]][e[0]] = 1

ans = 0

def make_permutation(li, r):
    if (r <= 0) or (r > len(li)):
        return []
    
    result = []
    _make_permutation(li, r, [], result)
    return result

def _make_permutation(li, r, out, result):
    if (r == 0):
        result.append(out)
        return

    for i, num in enumerate(li):
        _make_permutation(li[:i]+li[i+1:], r-1, out+[li[i]], result)

vs = list(range(1,N))
perm = make_permutation(vs, N-1)

for pe in perm:
    before = 0
    for num in pe:
        if (edge[before][num]):
            before = num
            continue
        else:
            break
    else:
        ans += 1

print(ans)
