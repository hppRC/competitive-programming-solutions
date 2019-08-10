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
def SR(n): return [sys.stdin.readline()[:-1] for i in range(n)]
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

d = defaultdict(int)

N = I()
S = SR(N)

for s in S:
    d[str(sorted(s))] += 1

ans = 0
for key in d.keys():
    value = d[key]
    if value >= 2 :
        ans += value*(value-1)//2
    
print(ans)