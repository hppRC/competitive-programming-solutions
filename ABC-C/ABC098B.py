N = int(input())
S = input()
count = 0
for i in range(1, N) :
    X = list(S[:i])
    Y = list(S[i:])
    count = max(count, len(list(set(X)&set(Y))))
print(count)