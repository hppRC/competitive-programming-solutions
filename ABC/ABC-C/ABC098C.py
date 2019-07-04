N = int(input())
S = input()

leader = S[0]
right = 0
left = 0

for i in range(1,N):
    right += 1 if (S[i]=="W") else 0

result = right

for i in range(1,N):
    if (S[i-1]=="E"):
        left += 1
    if (S[i]=="W"):
        right -= 1
    result = max(result, right+left)

print(N-result-1)


    