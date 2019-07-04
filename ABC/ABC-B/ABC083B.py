N, A, B = map(int, input().split())

num = [i for i in range(1,N+1)]
li = [sum(list(map(int, str(i)))) for i in range(1,N+1)]

result = 0
for i, j in zip(num, li):
    result += i if (j>=A and j <= B) else 0
print(result)