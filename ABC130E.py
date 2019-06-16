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

N, M = LI()
S = LI()
T = LI()

dp = [[1]*(N+1) for i in range(M+1)]

lcs = [[0]*(N+1) for i in range(M+1)]

for i in range(M):
    for j in range(N):
        lcs[i+1][j+1] = lcs[i][j]+1 if S[i]==T[j] else max(lcs[i][j+1], lcs[i+1][j]) 

for i in range(M):
    for j in range(N):
        dp[i+1][j+1] = dp[i][j] + lcs[i+1][j+1] if (S[i]==T[j]) else 
    
print(lcs)