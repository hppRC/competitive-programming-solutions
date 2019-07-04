import collections

N = int(input())
xy = [list(map(int, input().split())) for i in range(N)]

if (N==1):
    print(1)
    quit()

pq = []
cost = N

for xyi in xy:
    for xyj in xy:
        pq.append((xyi[0]-xyj[0], xyi[1]-xyj[1]))

count = collections.Counter(pq)
if (count.most_common()[0][0] == (0,0)):
    print(N - count.most_common()[1][1])
else:
    print(N - count.most_common()[0][1])

