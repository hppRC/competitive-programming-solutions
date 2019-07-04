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

T = I()
SS = [input() for i in range(T)]

ans = []

for S in SS:
    tmp_ans = 0
    i = 0
    if (S[-i-5:]=="kyoto") or (S[-i-5:]=="tokyo"):
        tmp_ans += 1
        i += 5
    else:
        i += 1
    while (i+5<=len(S)):
        if (S[-i-5:-i]=="kyoto") or (S[-i-5:-i]=="tokyo"):
            i += 5
            tmp_ans += 1
        else:
            i += 1
    ans.append(tmp_ans)
    
for i in ans:
    print(i)
