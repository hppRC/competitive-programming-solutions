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
a = LI()

dp = [[[-1]*(N+1) for i in range(N+1)] for j in range(N+1)]

def rec(i, j, k):
    if (dp[i][j][k] > -1):
        return dp[i][j][k]
    if (i==0) and (j==0) and (k==0):
        return 0
    res = N
    if (i>0):
        res += rec(i-1, j, k)*i
    if (j>0):
        res += rec(i+1, j-1, k)*j
    if (k>0):
        res += rec(i, j+1, k-1)*k
    res /= i+j+k
    dp[i][j][k] = res
    return res

print(rec(a.count(1), a.count(2), a.count(3)))