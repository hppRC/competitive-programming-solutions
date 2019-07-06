def solve(n ,l, r, A):
    res = 0
    for x in range(l, r+1):
        for i in range(n):
            if (not x%A[i]) :
                res += 0 if (i&1) else 1
                break
        else:
            res += 0 if (n&1) else 1
    return res


def main():
    ans = []
    while True:
        n, l, r = map(int, input().split())
        if (not n) and (not l) and (not r):
            break
        A = [int(input()) for i in range(n)]
        ans.append(solve(n,l,r,A))
    for i in ans:
        print(i)


main()