prime = []
is_prime = [1 for i in range(1000000)]


is_prime[0] = is_prime[1] = 0
for i in range(2,1000000):
    if (is_prime[i]):
        prime.append(i)
        for j in range(2*i,1000000,i):
            is_prime[j] = 0


def solve(a, d, n):
    count = 0
    for p in prime:
        if (p-a < 0):
            continue
        if (not (p-a)%d):
            count += 1
        if (count == n):
            return p
            
def main():
    ans = []
    while True:
        a, d, n = map(int, input().split())
        if (a==0) and (d==0) and (n==0):
            break
        ans.append(solve(a,d,n))

    for i in ans:
        print(i)

main()