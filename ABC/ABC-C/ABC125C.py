import fractions
"""
import math
import queue

N = int(input())
As = sorted(list(map(int, input().split())))

backet = [1 for i in range(As[-1]+1)]
ans = 1

for i, Ai in enumerate(As):
    for j in range(2,int(Ai/2)+1) :
        if (Ai % j == 0) :
            backet[j] += 1

flag = True
while flag :
    flag = False
    for i, bc in enumerate(backet):
        if (bc >= N) :
            backet[i] -= N
            ans *= (bc-1)
            flag = True
            continue

print(ans)
"""

N = int(input())
A = list(map(int, input().split()))


def L(i) :
    if (i-1 < 0) :
        return 0
    return fractions.gcd(L(i-1), A[i-1])

def R(i) :
    if (i >= N) :
        return 0
    return fractions.gcd(R(i+1), A[i])

M = 0
for i in range(N) :
    M = max(M, fractions.gcd(L(i), R(i+1)))


print(M)
