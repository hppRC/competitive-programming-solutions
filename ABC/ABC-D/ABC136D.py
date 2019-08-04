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

S = S()

Rend, Lend = 0, 0

ans = [0]*(len(S))

for i in range(len(S)-1):
    if (S[i]=='R' and S[i+1]=='L'):
        l, r = i, i+1
        for j in range(i+1,len(S)):
            if (S[j]=='L'):
                Lend = j
            else:
                break
        else:
            ans[l] = ((l-Rend+1)//2 + (l-Rend+1)&1) + (r-Lend+1)//2
            ans[r] = (l-Rend+1)//2 + ((r-Lend+1)//2 + (r-Lend+1)&1)

        ans[l] = ((l-Rend+1)//2) + ((l-Rend+1)&1) + ((Lend-r+1)//2)
        ans[r] = ((l-Rend+1)//2) + ((Lend-r+1)//2) + ((Lend-r+1)&1)
        Rend = Lend+1
        i = Rend

print(*ans)