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
edge = list(map(lambda x: [x[0]-1, x[1]-1], LIR(M)))
table = [[] for i in range(N)]
for e in edge:
    table[e[0]].append(e[1])
    table[e[1]].append(e[0])

checked = [0]*N

q = deque()
ans = 0

for i in range(N):
    if (not checked[i]):
        checked[i] = 1
        for v in table[i]:
            q.append((v, i))
    else:
        continue

    flag = True
    while q:
        v = q.pop()
        checked[v[0]] = 1
        for u in table[v[0]]:
            if (checked[u]) and (u != v[1]):
                flag = False
            elif (u != v[1]):
                q.append((u, v[0]))
    if (flag) :
        ans += 1

print(ans)

"""
for e in edge:
    utov[e[0]].append(e[1])

check = [0]*N

def dfs(v, before, check):
    check[v] = 1
    print(check)
    for i in utov[v]:
        if (check[i]) and (i != before):
            return False
        dfs(i, v, check)
    
    check[v] = 0
    return True

first_v = utov[edge[0][0]][0]
if (not dfs(first_v, first_v, check)):
    print("No")
else:
    print("Yes")
"""




    