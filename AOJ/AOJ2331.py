def main():
    N = int(input())
    ab = [list(map(int, input().split())) for i in range(N)]

    imos = [0]*(N+2)
    for a, b in ab:
        imos[a] += 1
        imos[min(b,N+1)] -= 1
    for i in range(N+1):
        imos[i+1] += imos[i]
    print(imos)

main()