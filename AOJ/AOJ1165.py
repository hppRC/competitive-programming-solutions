def solve(N, nd):
    dire = [(0,-1), (-1,0), (0,1), (1,0)]
    table = [[0]*(2*N+1) for i in range(2*N+1)]
    table[N][N] = 1
    position = [None]*N
    position[0] = (N, N)
    for i,ndi in enumerate(nd,start=1):
        n, d = ndi
        y, x = position[n]
        ny, nx = y+dire[d][0], x+dire[d][1]
        table[ny][nx] = 1
        position[i] = (ny, nx)

    left, right, top, bottom = 2*N, 0, 2*N, 0
    for i in range(2*N):
        for j in range(2*N):
            if (table[i][j]):
                left = min(left, j)
                right = max(right, j)
                top = min(top, i)
                bottom = max(bottom, i)
    return (right-left+1, bottom-top+1)




def main():
    ans = []
    while True:
        N = int(input())
        if (not N):
            break
        nd = [list(map(int, input().split())) for i in range(N-1)]
        ans.append(solve(N, nd))
    for i in ans:
        print(*i)
    
main()