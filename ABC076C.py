import sys

S_ = list(sys.stdin.readline()[:-1])
T = list(sys.stdin.readline()[:-1])

start_right = -1
for i in range(len(S_)):
    if (S_[i] == "?") or (S_[i] == T[0]):
        for j in range(1, len(T)):
            if (i+j >= len(S_)):
                break
            if (S_[i+j] == "?") or (S_[i+j] == T[j]):
                continue
            else:
                break
        else:
            start_right = i
    
for i in range(len(T)):
    S_[start_right + i] = T[i]

if (start_right != -1):
    for s in S_:
        print(s if (s != "?") else "a", end="")
    print()
else:
    print("UNRESTORABLE")