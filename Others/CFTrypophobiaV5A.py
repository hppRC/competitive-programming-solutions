def solve(N, xy, M, diredist):
    now = (10, 10)
    field = [[0]*21 for i in range(21)]
    field[now[0]][now[1]] = 1
    direction = {"N":(1, 0), "W":(0,-1), "S":(-1, 0), "E":(0, 1)}
    for i in range(M):
        dire = direction[diredist[i][0]]
        dist = int(diredist[i][1])
        for j in range(dist):
            field[now[0]+dire[0]][now[1]+dire[1]] = 1
            now = (now[0]+dire[0], now[1]+dire[1])
        
    for xi, yi in xy:
        if field[yi][xi]:
            continue
        else:
            return False
    
    return True
    

def main():
    ans = []
    while True:
        N = int(input())
        if (not N):
            break
        xy = [list(map(int, input().split())) for i in range(N)]
        M = int(input())
        diredist = [input().split() for i in range(M)]
        ans.append(solve(N, xy, M, diredist))
    
    for i in ans:
        print("Yes" if i else "No")


main()
