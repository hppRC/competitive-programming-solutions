N = int(input())
Vs = list(map(int, input().split()))
Cs = list(map(int, input().split()))

print(sum([v-c for v, c in zip(Vs, Cs) if v-c>=0]))