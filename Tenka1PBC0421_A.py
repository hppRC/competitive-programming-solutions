A, B, C = list(map(int, input().strip().split()))

if (A>B) :
    tmp = A
    A = B
    B = tmp

if (A < C and B > C) :
    print("Yes")
else :
    print("No")