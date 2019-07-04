def indexOfLeftWhite(S) :
    for i in range(len(S)):
        if (S[i] == ".") :
            return i
    return -1

def indexOfLeftBlack(S) :
    for i in range(len(S)):
        if (S[i] == "#") :
            return i
    return -1

N = int(input())
S = input()

counter = 0

if (S[-2:] == "#.") :
    S = S[:-1] + "#"
    counter += 1
"""
else:
    for i in range(1,len(S))[::-1] :
        if (S[i-1:i+2] == "#.#") :
            S = S[:i-1] + "###" + S[i+2:]
            counter += 1
        elif (S[i-1:i+2] == "#..") :
            S = S[:i-1] + "..." + S[i+2:]
            counter += 1
"""

for i in range(len(S)) :
    tmp = S.replace("#..", "...") 
    L = tmp.replace("#.#", "###")

for i in range(len(S)) :
    if (S[i] != L[i]) :
        counter += 1

print(counter)