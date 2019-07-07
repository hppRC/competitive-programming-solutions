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
intINF = int(1e15)
mod = 1000000007
dire4 = [(1,0), (0,1), (-1,0), (0,-1)]
dire8 = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

N = I()
a = LI()

dp = [[intINF]*(N+1) for i in range(N+1)]
acc = [0]*(N+1)
for i in range(N):
    acc[i+1] = acc[i] + a[i]

for i in range(N-1):
    dp[i][i+2] = a[i]+a[i+1]
    dp[i][i+1] = 0
    dp[i][i] = 0
dp[N-1][N] = 0
dp[N-1][N-1] = 0
dp[N][N] = 0

for l in range(N-1)[::-1]:
    for r in range(l, N+1):
        for k in range(l+1,r):
            dp[l][r] = min(dp[l][r], dp[l][k]+dp[k][r]+(acc[r]-acc[l]))

print(dp[0][N])