import math

a, b = input().split()
n = int(a+b)

sq = int(math.sqrt(n))
print("Yes" if (sq*sq==n) else "No")