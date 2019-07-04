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

N, K = LI()

def fact(n):
    if (n==0) or (n==1):
        return 1
    ret = 1
    for i in range(1, n+1):
        ret *= i
    return ret


for i in range(1, K+1):
    if (N-K-i+1>= 0):
        t1 = math.factorial(N-K+1)
        t2 = math.factorial(i)
        t3 = math.factorial(N-K-i+1)
        tmp1 = (t1//(t2*t3))%mod

        t4 = math.factorial(K-1)
        t5 = math.factorial(i-1)
        t6 = math.factorial(K-i)
        tmp2 = (t4//(t5*t6))%mod

        print(tmp1 * tmp2 % mod)
    else:
        print(0)