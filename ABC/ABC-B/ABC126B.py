S = input()

front = int(S[:2])
back = int(S[2:])

if (front==0) and (back==0):
    print("NA")
elif (front==0) and (back <= 12):
    print("YYMM")
elif (front <= 12) and (back==0):
    print("MMYY")
elif (front <= 12 and front!=0) and (back <= 12 and back!=0):
    print("AMBIGUOUS")
elif (front > 12) and (back <= 12 and back!= 0):
    print("YYMM")
elif (front <= 12 and front!=0) and (back > 12):
    print("MMYY")
else :
    print("NA")