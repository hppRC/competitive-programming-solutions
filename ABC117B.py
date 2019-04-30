N = int(input())
edges = sorted(list(map(int, input().split())))

print( "Yes" if edges[-1] < sum(edges[:-1]) else "No" )