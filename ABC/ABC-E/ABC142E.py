#!usr/bin/env python3
from collections import defaultdict
from collections import deque
from heapq import heappush, heappop
from functools import reduce
import sys
import math
import bisect
import random
def I(): return int(sys.stdin.readline())
def F(): return float(sys.stdin.readline())
def S(): return input()
def LI(): return list(map(int, sys.stdin.readline().split()))
def LI_(): return [int(x)-1 for x in sys.stdin.readline().split()]
def LF(): return list(map(float, sys.stdin.readline().split()))
def LS():return sys.stdin.readline().split()
def SR(n): return [list(sys.stdin.readline())[:-1] for i in range(n)]
def IR(n): return [int(sys.stdin.readline()) for i in range(n)]
def LIR(n): return [list(map(int, sys.stdin.readline().split())) for i in range(n)]
def LSR(n): return [list(map(list, sys.stdin.readline().split())) for i in range(n)]
_gcd = lambda x, y: _gcd(y, x%y) if (x%y) else y
_lcm = lambda x, y: x*y // _gcd(x, y)
def gcd(*numbers):
    return reduce(_gcd, numbers)
def lcm(*numbers):
    return reduce(_lcm, numbers)
sys.setrecursionlimit(1000000)
INF = float('inf')
MOD = 1000000007
dire4 = [(1,0), (0,1), (-1,0), (0,-1)]
dire8 = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

N, M = LI()

a = []
b = []
c = []

for i in range(M):
    ai, bi = LI()
    ci = LI()
    a.append(ai)
    b.append(bi)
    c.append(ci)

dp = [[MOD] * (1 << N)]*(M+1)

for i in range(M+1):
    dp[i][0] = 0

for i in range(M):
    for j in range(1<<N):
        boxSets = 0
        for cij in c[i]:
            boxSets |= 1 << (cij - 1)
        dp[i][j | boxSets] = min(dp[i][j | boxSets], dp[i][j] + a[i])

print(-1 if (dp[M][(1 << N)-1] == MOD ) else dp[M][(1 << N)-1])


