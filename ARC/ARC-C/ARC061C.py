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

S = S()
ans = 0

for i in range(1<<(len(S)-1)):
    tmp_ans = 0

    for j in range(len(S)-1):
        if (i & 1):
            tmp_ans *= 10
            tmp_ans += int(S[j])
            ans += tmp_ans
            tmp_ans = 0
        else:
            tmp_ans *= 10
            tmp_ans += int(S[j])
       
        i >>= 1
    
    if (not tmp_ans):
        ans += int(S[-1])
    else:
        tmp_ans *= 10
        tmp_ans += int(S[-1])
        ans += tmp_ans

print(ans)
