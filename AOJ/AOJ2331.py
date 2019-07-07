def solve(N, ab):
    imos = [1]*(N+2)
    for a, b in ab:
        for i in range(a,min(b+1,N+2)):
            imos[i] += 1
    ret = 1
    for i, a in enumerate(imos):
        if (a>i):
            ret = max(ret, i)
    return ret-1


def main():
    N = int(input())
    ab = [list(map(int, input().split())) for i in range(N)]
    print(solve(N, ab))

main()

        
