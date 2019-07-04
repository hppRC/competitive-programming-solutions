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

ABCD = S()

for i in range(1<<3):
    tmp = 0
    bit = i

    for j in range(3):
        if (bit & 1):
            tmp += int(ABCD[-j-1])
        else:
            tmp -= int(ABCD[-j-1])
        bit >>= 1
        
    tmp += int(ABCD[0])
    if (tmp == 7):
        ans = i
        break


operands = []
for i in range(3):
    if (ans & 1):
        operands.append("+")
    else:
        operands.append("-")
    ans >>= 1

for i, ope in enumerate(operands[::-1]):
    print(ABCD[i], end="")
    print(ope, end="")
print(ABCD[3], end="")
print("=7")