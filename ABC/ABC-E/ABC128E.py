N, Q = map(int, input().split())
STX = sorted([list(map(int, input().split())) for i in range(N)], key=lambda x: (x[2], x[0]-x[2]))
D = sorted([(int(input()), i) for i in range(Q)], key=lambda x: x[0])
distance = [-1 for i in range(Q)]
mapping = []
for s,t,x in STX:
    mapping.append((s-x, x))
    mapping.append((t-x, x))
i = 0 #mapping
j = 0 #Q

while True:
    if (i >= 2*N or j >= Q):
        break
    d = D[j][0]
    if (d >= mapping[i][0] and d < mapping[i+1][0]):
        distance[D[j][1]] = min(distance[D[j][1]], mapping[i][1]) if distance[D[j][1]]>=0 else mapping[i][1]
        j += 1
    elif (d <= mapping[i][0]):
        j += 1
    elif (d >= mapping[i+1][0]):
        i += 2   

for i in distance:
    print(i)