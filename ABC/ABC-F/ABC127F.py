Q = int(input())
Query = [list(map(int, input().split())) for q in range(Q)]
mean = 0
count = 0

f = lambda x: 0
for q in Query:
    if q[0]==1:
        f = lambda x: f(x) + abs(x-q[1]) + q[2]
        mean *= count
        count += 1
        mean += q[1]
        mean /= count
    else:
        print(int(x))
        print()
