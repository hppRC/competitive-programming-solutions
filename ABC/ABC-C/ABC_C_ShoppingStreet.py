N = int(input())
F = [list(map(int, input().strip().split())) for i in range(N)]
P = [list(map(int, input().strip().split())) for i in range(N)]
maxProfit = -1000000000


for i in range(1,1024):
    openTime = list(map(int, list("{:0>10}".format(bin(i)[2:]))))
    thisProfit = 0
    for n in range(N):
        shopOpneTime = F[n]
        shopProfit = P[n]
        counter = 0
        for j in range(10):
            if((shopOpneTime[j])and(openTime[j])):
                 counter += 1
        thisProfit += shopProfit[counter]
    maxProfit = max(thisProfit, maxProfit)

print(maxProfit)
