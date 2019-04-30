N = int(input())
A = sorted(list(map(int, input().split())))
print(A[-1] - A[0])