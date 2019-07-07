def solve(H, W, field):
    

def main():
    ans = []
    N = int(input())
    for i in range(N):
        H, W = list(map(int, input().split()))
        field = [input() for j in range(H)]
        ans.append(solve(H, W, field))

    print(ans)