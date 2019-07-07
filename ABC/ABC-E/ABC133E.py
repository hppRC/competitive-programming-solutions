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


N, K = LI()
ab = LIR(N-1)

#ある頂点からある頂点への隣接リスト
utov = [[] for i in range(N+1)]

for a, b in ab:
    utov[a].append(b)
    utov[b].append(a)

#キュー作る
q = deque()
#ある頂点の塗り方を書き込む配列
bfs = [-1]*(N+1)

#１を木の根と考える
#根の部分は好きに塗れるからK通りの塗り方がある
bfs[1] = K

#根の子たちは、根と違う色かつ他のことも違う色に塗らなくてはいけない
#ある頂点を塗るたびにchildを+1して表現
child = 0
for v in utov[1]:
    if (bfs[v] == -1):
        child += 1
        bfs[v] = K - child
        q.append(v)
        #塗ったところをキューに突っ込む

#キューを回す
while q:
    u = q.popleft()
    #根の子の子からは、その頂点の親(根の子)とさらにその親(根)が塗ってある状態なので、
    #頂点を塗れるとしたらK-2通りがデフォ
    child = 1
    for v in utov[u]:
        if (bfs[v] == -1):
            child += 1
            #ある頂点の最初の子はK-2通り
            #ある頂点の二つ目の子はK-3通り...って感じ
            bfs[v] = K - child
            q.append(v)

#答え格納用の変数
ans = 1

#木の全頂点を見ながら、何通りに塗れるかを掛け合わせていく
#modもとる
for pattern in bfs[1:]:
    ans = ans*pattern % mod

print(ans%mod) #コーナーケース対策にmodとっておく




"""
def search_colored_num(u):
    q = deque()
    q.append(u)
    dist = [-1]*(N+1)
    dist[u] = 0
    ret = 0
    while q:
        u = q.popleft()
        for v in utov[u]:
            if (dp[v]) and (dist[u]<=1) and (dist[v] == -1):
                ret += 1
                dist[v] = dist[u] + 1 
                q.append(v)
    return ret

N, K = LI()

ab = LIR(N-1)

utov = [[] for i in range(N+1)]
dp = [0]*(N+1)

for a, b in ab:
    utov[a].append(b)
    utov[b].append(a)

q = deque()
q.append(1)

while q:
    u = q.popleft()
    num = search_colored_num(u)
    dp[u] = K-num
    for v in utov[u]:
        if (not dp[v]):
            q.append(v)

check = [False]*(N+1)
q.append(1)
ans = 1
while q:
    u = q.popleft()
    ans *= dp[u]
    ans %= mod
    check[u] = True
    for v in utov[u]:
        if (dp[v]!=0) and (not check[v]):
            q.append(v)
print(ans)
"""