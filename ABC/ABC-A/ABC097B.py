import math
X = int(input())
max_num = 1

for i in range(2, int(math.sqrt(X))+1) :
    for j in range(2, int(math.log2(X))+1) :
        if (i**j <= X):
            max_num = max(max_num, i**j)
        else:
            break
print(max_num)