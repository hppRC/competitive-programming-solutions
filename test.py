def bfs(sy=0, sx=0, c=[['.']], gy=0, gx=0, blocks=['#'], return_maximum_distance=False, dire=[(1,0), (0,1), (-1,0), (0,-1)]):
    H = len(c)
    W = len(c[0])
    dist = [[1000000007]*W for i in range(H)]

    q = deque()

    if (type(sy)==list) and (type(sx)==list):
        for syi, sxi in zip(sy, sx):
            dist[syi][sxi] = 0
            q.append((syi, sxi))
    else:
        dist[sy][sx] = 0
        q.append((sy, sx))

    if (return_maximum_distance):
        while q:
            y, x = q.popleft()
            for dy, dx in dire:
                if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                    if (c[y+dy][x+dx] in blocks):
                        continue
                    elif (dist[y+dy][x+dx] > dist[y][x]+1):
                            dist[y+dy][x+dx] = dist[y][x]+1
                            q.append((y+dy, x+dx))
        maximum_distance = 0
        for i in range(H):
            for j in range(W):
                if (dist[i][j] < 1000000007):
                    maximum_distance = max(dist[i][j], maximum_distance)
        return maximum_distance

    else:
        while q:
            y, x = q.popleft()
            for dy, dx in dire:
                if (y+dy<H) and (y+dy>=0) and (x+dx<W) and (x+dx>=0):
                    if (c[y+dy][x+dx] in blocks):
                        continue
                    else:
                        if (y+dy==gy) and (x+dx==gx):
                            return (dist[y][x]+1)
                        elif (dist[y+dy][x+dx] > dist[y][x]+1):
                            dist[y+dy][x+dx] = dist[y][x]+1
                            q.append((y+dy, x+dx))
        return -1

print([[1009]*3 for i in range(10)])