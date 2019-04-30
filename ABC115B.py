N = int(input())
items = sorted([int(input()) for i in range(N)])
print(sum(items)-int(items[-1]/2))