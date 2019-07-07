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

A_sum = 0
A_acc_odd = [0]*(N//2+1)
A_acc_even = [0]*(N//2+2)

for i in range(N):
    A_sum += A[i]
    if (i&1):
        A_acc_odd[i//2+1] = A_acc_odd[i//2] + A[i]
    else:
        A_acc_even[i//2+1] = A_acc_even[i//2] + A[i]

x = [None]*N

for i in range(N):
    if (i%2):
        x[i] = A_sum - 2*((A_acc_odd[i//2]-A_acc_odd[0]) + (A_acc_even[-1] - A_acc_even[i//2+1]))
    else:
        x[i] = A_sum - 2*((A_acc_even[i//2]-A_acc_even[0]) + (A_acc_odd[-1] - A_acc_odd[i//2]))
print(*x)