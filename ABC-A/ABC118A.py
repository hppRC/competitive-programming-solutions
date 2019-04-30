A, B = list(map(int, input().split()))
print(A+B if B%A==0 else B-A)