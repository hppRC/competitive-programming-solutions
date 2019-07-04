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

dire = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]

def solve(W, H, c):
    ret = 0
    checked = [[0]*W for i in range(H)]
    q = deque()
    for i in range(H):
        for j in range(W):
            if (c[i][j]) and (not checked[i][j]):
                q.append((i, j)) #((now),(before))
                while q:
                    v = q.pop()
                    nowy, nowx = v
                    checked[nowy][nowx] = 1
                    for dy, dx in dire:
                        if (nowy+dy<H) and (nowy+dy>=0) and (nowx+dx<W) and (nowx+dx>=0):
                            if (c[nowy+dy][nowx+dx]) and (not checked[nowy+dy][nowx+dx]):
                                q.append((nowy+dy, nowx+dx))
                ret += 1
    return ret


def main():
    ans = []
    while True:
        W, H = LI()
        if (W==0) and (H==0):
            break
        c = LIR(H)
        ans.append(solve(W, H, c))
    for i in ans:
        print(i)


main()