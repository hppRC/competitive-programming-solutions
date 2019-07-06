from collections import deque

def solve(W, H, walls):
    edge = [[[] for i in range(W)] for j in range(H)]
    for i in range(H):
        for j in range(W-1):
            if (not walls[2*i][j]):
                edge[i][j].append((i, j+1))
                edge[i][j+1].append((i,j))
    for i in range(H-1):
        for j in range(W):
            if (not walls[2*i+1][j]):
                edge[i][j].append((i+1, j))
                edge[i+1][j].append((i,j))

    q = deque()
    q.append((0, 0))
    dist = [[-1]*W for i in range(H)]
    dist[0][0] = 1

    while q:
        y, x = q.popleft()
        for ny, nx in edge[y][x]:
            if (dist[ny][nx] == -1):
                dist[ny][nx] = dist[y][x] + 1
                q.append((ny, nx))


    return dist[H-1][W-1] if (dist[H-1][W-1] != -1) else 0

"""
def solve(W, H, walls):
    dire = [(1,0), (0,1), (-1,0), (0,-1)]
    field = [[1]*(2*W) for i in range(2*H)]
    for i in range(0,2*H,2):
        vertical_wall = walls[i]
        for j in range(W-1):
            if (vertical_wall[j]):
                field[i][j*2+1] = field[i+1][j*2+1] = 0

    for i in range(1,2*H-1,2):
        horizontal_wall = walls[i]
        for j in range(W):
            if (horizontal_wall[j]):
                field[i][j*2] = field[i][j*2+1] = 0

    q = deque()
    dist = [[-1]*(2*W) for i in range(2*H)]
    q.append((0,0))
    dist[0][0] = 0

    while q:
        y, x = q.popleft()
        for dy, dx in dire:
            if (y+dy<2*H) and (y+dy>=0) and (x+dx<2*W) and (x+dx>=0):
                if (field[y+dy][x+dx]) and (dist[y+dy][x+dx] == -1):
                    dist[y+dy][x+dx] = dist[y][x] + 1
                    q.append((y+dy, x+dx))
    res = dist[2*H-1][2*W-1]//2

    for i in field:
        for j in i:
            print("." if j else "#", end="")
        print()

    ans_dist = [[0]*W for i in range(H)]
    for i in range(H):
        for j in range(W):
            ans_dist[i][j] = max(dist[2*i][2*j], dist[2*i][2*j+1], dist[2*i+1][2*j], dist[2*i+1][2*j+1]) // 2
    for i in ans_dist:
        print(i)

    #for i in dist:
       # print(i)

    return res if (res != -1) else 0

"""
def main():
    ans = []
    while True:
        W, H = map(int, input().split())
        if (W==0) and (H==0):
            break
        walls = [list(map(int, input().split())) for i in range(2*H-1)]
        ans.append(solve(W, H, walls))
    
    for i in ans:
        print(i)

main()