N, C = list(map(int, input().strip().split()))

dataSet = [list(map(int, input().strip().split())) for i in range(N)]

for data in dataSet:
    data[0] -= 0.5

sortedData = sorted(dataSet, key=lambda x:x[1])


num = 0
sTime = 0
recoder = 1

while(True) :
    if(len(sortedData)==0) :
        break
    if(num < len(sortedData)) :
        if(sortedData[num][0] >= sTime) :

            sTime = sortedData.pop(num)[1]
        else :
            num += 1
    else :
        recoder += 1
        num = 0
        sTime = 0


print(recoder)
