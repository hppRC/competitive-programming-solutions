def solve(n, score):
    score = sorted(score)
    return sum(score[1:-1])//len(score[1:-1])

def main():
    ans = []
    while True:
        n = int(input())
        if (n == 0):
            break
        score = [int(input()) for i in range(n)]
        ans.append(solve(n, score))
    
    for i in ans:
        print(i)

main()