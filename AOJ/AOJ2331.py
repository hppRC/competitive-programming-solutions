def main():
    N = int(input())
    ab = [list(map(int, input().split())) for i in range(N)]

    imos = [0]*(N+3)
    for a, b in ab:
        imos[min(a,N+2)] += 1
        imos[min(b+1,N+2)] -= 1
    for i in range(N+1):
        imos[i+1] += imos[i]
    #print(imos)

    ans = 0
    for i, imo in enumerate(imos):
        if (imo+1 >= i):
            ans = i-1
    print(ans)

main()