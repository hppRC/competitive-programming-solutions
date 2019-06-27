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

n, m = LI()
c = LI()
c = sorted(c)

dp = [n+1]*(n+1)
dp[0] = 0

for i in range(n+1):
    for coin in c:
        if (i+coin <= n) and (dp[i+coin] > dp[i]+1):
            dp[i+coin] = dp[i]+1

print(dp[n])