N = int(input())
A = sorted(list(map(int, input().split())))

if (A[0] >= 0):
    print(sum(A)-A[0]*2)
elif (A[-1] <= 0):
    print(-sum(A)+A[-1]*2)
else:
    print(sum(list(map(abs, A))))

for i in range(1, N-1):
    if (A[i] < 0):
        print(A[N-1], A[i])
        A[N-1] -= A[i]
    else:
        print(A[0], A[i])
        A[0] -= A[i]
    print(A[N-1], A[0])