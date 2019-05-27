N, K = map(int, input().split())
result = 0

def check(i, K):
    count = 0
    while (K > i):
        i *= 2
        count += 1
    return count

for i in range(1,N+1):
    count = check(i, K)
    dec = 1
    for j in range(count):
        dec /= 2.0
    result += dec

print(round(result/N, 12))