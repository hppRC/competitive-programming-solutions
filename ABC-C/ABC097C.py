s = input()
K = int(input())

N = len(s)

li = []
for i in range(N):
    for j in range(5):
        if (i+j >= N):
            break
        li.append(s[i:i+j+1])
    li = sorted(list(set(li)))[:6]
    

print(li[K-1])

