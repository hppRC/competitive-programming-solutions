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

N, K = LI()
a = LI()
"""

dp = np.zeros(K+1, dtype=np.int64)
dp[0:a[0]+1] = 1

for i in range(1,N):
    acc = np.cumsum(dp)
    for j in range(K+1)[::-1]:
        if (j-a[i]<=0):
            dp[j] = acc[j]%mod
        else:
            dp[j] = (acc[j] - acc[j-a[i]-1])%mod

print(dp[K])

"""
dp = [0]*(K+1)
acc = [0]*(K+2)

for i in range(K+1):
    if (i <= a[0]):
        dp[i] =  1
    acc[i+1] = acc[i] + dp[i]

for i in range(1,N):
    #for i in range(K):
        #acc[i+1] = acc[i] + dp[i]
    #print(acc)
    for j in range(K+1)[::-1]:
        dp[j] = (acc[j+1] - acc[max(j-a[i], 0)])%mod

    acc[0] = 0
    for j in range(K+1):
        acc[j+1] = (acc[j] + dp[j])%mod

print(dp[K])

"""
dp = [[0]*(K+1) for i in range(N+1)]

for i in range(K+1):
    if (i > a[0]):
        break
    dp[1][i] =  1
for i in range(1,N+1):
    dp[i][0] = 1


for i in range(1,N):
    for j in range(K+1):
        res = 0
        for k in range(a[i]+1):
            if (k>j):
                break
            res += dp[i][j-k]
        dp[i+1][j] = res%mod


for i in dp:
    print(i)
"""