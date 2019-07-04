A, B = map(int, input().split())
S = input()

flag = True

if (S[A]!="-"):
    flag = False
if (not S[:A].isdigit()):
    flag = False
if (not S[A+1:A+B+1].isdigit()):
    flag = False

print("Yes" if flag else "No")