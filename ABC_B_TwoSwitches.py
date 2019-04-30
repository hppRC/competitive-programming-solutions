A, B, C, D = list(map(int, input().strip().split()))

time= 0
if(B <= C):
    time = 0
elif(D <= A):
    time = 0
elif(A <= C)and(B <= D):
    time = B - C
elif(A <= C)and(D <= B):
    time = D - C
elif(C <= A)and(D <= B):
    time = D - A
elif(C <= A)and(B <= D):
    time = B - A
print(time)
