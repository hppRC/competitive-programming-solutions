N = int(input())
A = list(map(int, input().split()))

print(sum(list(map(lambda ai: ai-1, A))))
