from collections import deque

def solve(W, H, field):
    dire = [(1,0), (0,1), (-1,0), (0,-1)]
    for i in range(H):
        for j in range(W):
            if (field[i][j]=="@"):
                s = (i,j)
    check = [[0]*W for i in range(H)]
    q = deque()
    q.append(s)
    check[s[0]][s[1]] = 1
    ret = 1
    while q:
        y, x = q.popleft()
        for dy, dx in dire:
            if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                if (field[y+dy][x+dx] == ".") and (not check[y+dy][x+dx]):
                    q.append((y+dy, x+dx))
                    check[y+dy][x+dx] = 1
                    ret += 1
    return ret

def main():
    ans = []
    while True:
        W, H = list(map(int, input().split()))
        if (not W) and (not H):
            break
        field = [input() for i in range(H)]
        ans.append(solve(W, H, field))
    for i in ans:
        print(i)
    
main()