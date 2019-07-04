#!usr/bin/env python3
from collections import defaultdict
from collections import deque
from heapq import heappush, heappop
import sys
import math
import bisect
import random
import gc
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
P = IR(N)
P.append(0)
P = sorted(P)

def dart4(N ,M, P):
    tmp_scores = []

    if (P[0] > M):
        return 0
        
    for i in range(N+1):
        for j in range(i, N+1):
            if (P[i]+P[j]==M):
                return P[i]+P[j]
            tmp_scores.append(P[i]+P[j])

    tmp_scores = sorted(tmp_scores)

    ans = 0
        
    for i in range(N+1):
        for j in range(i, N+1):
            point = bisect.bisect_right(tmp_scores, M-P[i]-P[j])
            if (point):
                ans = max(ans, tmp_scores[point-1]+P[i]+P[j])
            if (ans == M):
                break
        else:
            continue
        break

    return ans

print(dart4(N, M, P))


"""
def dart3(N, M, P):
    tmp_scores = []
        
    for i in range(N):
        for j in range(i, N):
            if (P[i]+P[j]==M):
                return P[i]+P[j]
            tmp_scores.append(P[i]+P[j])

    tmp_scores = sorted(tmp_scores)

    ans = 0

    for i in range(N):
        point = bisect.bisect_right(tmp_scores, M-P[i])
        if (point):
            ans = max(ans, tmp_scores[point-1]+P[i])
        if (ans == M):
            break

    del tmp_scores
    gc.collect()

    return ans

def dart2(N, M, P):
    ans = 0

    for i in range(N):
        point = bisect.bisect_right(P, M-P[i])
        if (point):
            ans = max(ans, P[point-1]+P[i])
        if (ans == M):
            break

    return ans

def dart1(N, M, P):
    return P[bisect.bisect_left(P, M)-1] if bisect.bisect_left(P, M) else 0

print(max(dart4(N, M, P), dart3(N ,M, P), dart2(N, M, P), dart1(N, M, P)))
"""