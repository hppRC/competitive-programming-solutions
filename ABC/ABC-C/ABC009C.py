import sys
from collections import defaultdict

N, K = list(map(int, sys.stdin.readline().split()))
S = list(sys.stdin.readline()[:-1])

def calc_cost(provided, rest):
    ret = 0
    for ch in list(rest.keys()):
        ret += abs(provided[ch] - rest[ch])
    return ret//2


provided = defaultdict(int)
rest = defaultdict(int)

for ch in S:
    provided[ch] += 1
    rest[ch] += 1

T = []
i = 0
past_cost = 0
rest_cost = 0
while True:
    for ch in sorted(list(rest.keys())):
        if (rest[ch]):
            rest[ch] -= 1
            provided[S[i]] -= 1
            rest_cost = calc_cost(provided, rest)
            past_cost += (S[i] != ch)
            
            if (rest_cost + past_cost > K):
                rest[ch] += 1
                provided[S[i]] += 1
                past_cost -= (S[i] != ch)
                continue
            else:
                T.append(ch)
                i += 1
                break
    else:
        break

print("".join(T))