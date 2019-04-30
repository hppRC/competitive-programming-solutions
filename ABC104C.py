D, G = list(map(int, input().split()))
prob = [list(map(int, input().split())) for i in range(D)]

min_count = 1e9

for mask in range(1<<D):
    count = 0
    score = 0
    rest = -1
    for i in range(D):
        if((mask>>i) & 1):
            score += prob[i][0]*100*(i+1) + prob[i][1]
            count += prob[i][0]
        else:
            rest = i

    for i in range(prob[rest][0]):
        if (score < G):
            score += (rest+1)*100
            count += 1
        else:
            min_count = min(min_count, count)
            break

print(min_count)