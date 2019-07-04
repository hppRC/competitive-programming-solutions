S = input()
K = int(input())
if (K <= len(S)) :
    if (S[:K] == '1'*K) :
        print('1')
    else:
        for ch in list(S):
            if (ch != '1'):
                print(ch)
                break
else :
    for ch in list(S):
        if (ch != '1'):
            print(ch)
            break