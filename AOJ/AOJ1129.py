def solve(n, r, pc):
    deck = [n+1-i for i in range(1,n+1)]
    for p, c in pc:
        deck = deck[p-1:p+c-1] + deck[0:p-1] + deck[p+c-1:]
    return deck[0]

def main():
    ans = []
    while True:
        n, r = map(int, input().split())
        if (n==0) and (r==0):
            break
        pc = [list(map(int, input().split())) for i in range(r)]
        ans.append(solve(n, r, pc))
    
    for i in ans:
        print(i)
    
main()