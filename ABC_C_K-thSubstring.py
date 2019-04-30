s = input()
K = int(input())
length = len(s)

substringList = []
for i in range(1,K+1) :
    for j in range(length-i+1) :
        substringList.append(s[j:j+i])


substringList = sorted(list(set(substringList)))
print(substringList[K-1])
