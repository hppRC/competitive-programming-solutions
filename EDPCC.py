#!usr/bin/env python3
from collections import defaultdict
from collections import deque
from heapq import heappush, heappop
from functools import reduce
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
_gcd = lambda x, y: _gcd(y, x%y) if (x%y) else y
_lcm = lambda x, y: x*y // _gcd(x, y)
def gcd(*numbers):
    return reduce(_gcd, numbers)
def lcm(*numbers):
    return reduce(_lcm, numbers)
def factorial(n):
    if (n==0) or (n==1):
        return 1
    ret = 1
    for i in range(2, n+1):
        ret *= i
    return ret
def combination(n, k):
    return factorial(n)//factorial(k)//factorial(n-k)
sys.setrecursionlimit(1000000)
mod = 1000000007
dire4 = [(1,0), (0,1), (-1,0), (0,-1)]
dire8 = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

N = I()
abc = LIR(N)

dp = [[0]*(N+1) for i in range(3)]
A, B, C = 0, 1, 2

for i in range(N):
    a, b, c = abc[i]
    dp[A][i+1] = max(dp[B][i], dp[C][i]) + a
    dp[B][i+1] = max(dp[A][i], dp[C][i]) + b
    dp[C][i+1] = max(dp[A][i], dp[B][i]) + c

print(max(dp[A][N], dp[B][N], dp[C][N]))