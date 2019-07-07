def solve(s):
    ret = [s]
    
    for i in range(len(s)+1):
        ret.append(s[:i]+s[i:][::-1])
        ret.append(s[:i][::-1]+s[i:])
        ret.append(s[:i][::-1]+s[i:][::-1])
        ret.append(s[i:]+s[:i])
        ret.append(s[i:][::-1]+s[:i])
        ret.append(s[i:]+s[:i][::-1])
        ret.append(s[i:][::-1]+s[:i][::-1])
    
    return len(set(ret))

def main():
    ans = []
    m = int(input())
    for i in range(m):
        ans.append(solve(input()))
    for i in ans:
        print(i)

main()
