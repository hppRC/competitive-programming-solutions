num = list(map(int, input().split()))
K = int(input())
print(sum(num) - max(num) + max(num)*(2**K))