from collections import deque

dire = [(1,0),(0,1),(-1,0),(0,-1)]

def search(y, x, field, check):
    ret = [(y,x)]
    check[y][x] = 1
    q = deque()
    q.append((y,x))
    while q:
        y, x = q.popleft()
        for dy, dx in dire:
            if (y+dy<5) and (y+dy>=0) and (x+dx<5) and (x+dx>=0):
                if (not check[y+dy][x+dx]) and (field[y+dy][x+dx] == field[y][x]):
                    ret.append((y+dy, x+dx))
                    check[y+dy][x+dx] = 1
                    q.append((y+dy), (x+dx))
    return ret
            

def vanish(field, combo):
    check = [[0]*5 for i in range(5)]
    vanish_v = [[0]*5 for i in range(5)]
    score = 0
    for y in range(5):
        for x in range(5):
            if (not check[y][x]):
                v = search(y, x, field, check)
                if (len(v)>=3):
                    vanish_v = choose(v, vanish_v)
    for y in range(5):
        for x in range(5):
            if (vanish_v[y][x]):
                score += field[y][x]*combo
                field[y][x] = 0 
    return score


def choose(v, vanish_v):
    ret_hor = [None]*5
    ret_ver = [None]*5
    for i in range(5):
        ret_hor[i] = [x for x in v if x[0]==i]
        ret_ver[i] = [x for x in v if x[1]==i]
    for i in range(5):
        if (len(ret_hor[i])>=3):
            for uy, ux in ret_hor[i]:
                vanish_v[uy][ux] = 1 
        if (len(ret_ver[i]>=3)):
            for uy, ux in ret_ver[i]:
                vanish_v[uy][ux] = 1
    return vanish_v

    
def drop(field):
    for y in range(5):
        for x in range(5):
            if (not field[y][x]):
                for i in range(i,5):
                    if (y+i>=5):
                        break
                    elif (field[i+y][x]):
                        field[y][x] = field[i+y][x]
    return field

def puyo(y, x, field, combo):
    ret = vanish(y, x)
    for dy, dx in dire:
        swapped_field = field.copy()
        ret = max(puyo(y+dy, x+dx, field, combo))
        


def solve(N, field, score):
    combo = 1
    score = 0


def main():
    ans = []
    while True:
        N = int(input())
        if (N==-1):
            break
        field = [list(map(int, input().split())) for i in range(5)]
        score = list(map(int, input().split()))
        ans.append(solve(N, field, score))
    for i in ans:
        print(i)


main()