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
mod = 1000000007
dire4 = [(1,0), (0,1), (-1,0), (0,-1)]
dire8 = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

N = I()
A = LI()

dp = [[[0]*(N+1) for i in range(N+1)] for j in range(N+1)]
dp[1][0][0] = 1/N
for i in range(N+1):
    for j in range(N+1):
        for k in range(N+1):
            if (not i) and (not j) and (not k):
                continue
            res = 0
            if (i>0):
                res += i*dp[i-1][j][k]
            if (j>0) and (i<N):
                res += j*dp[i+1][j-1][k]
            if (k>0) and (j<N):
                res += k*dp[i][j+1][k-1]
            dp[i][j][k] = res/(i+j+k)

print(dp[A.count(1)][A.count(2)][A.count(3)])
