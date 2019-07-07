import sys

sys.setrecursionlimit(1000000)


class St():
    def __init__(self, s):
        self.s = s
        self.i = 0

def expression(st, num):
    while (not st.s[st.i]=="."):
        if (st.s[st.i] == "("):
            if (not factor1(st)):
                return False

        elif (st.s[st.i] == "["):
            if (not factor2(st)):
                return False

        elif (not st.s[st.i] in [")", "]"]):
            st.i += 1

        else:
            if ((num==1) and (st.s[st.i]==")")) or ((num==2) and (st.s[st.i]=="]")):
                break
            else:
                return False
        if (st.i >= len(st.s)):
            return False
    return True
    
    
def factor1(st):
    st.i += 1
    if (st.s[st.i]=="."):
        return False
    if (expression(st, 1)):
        st.i += 1
        return True
    return False

def factor2(st):
    st.i += 1
    if (st.s[st.i]=="."):
        return False
    if (expression(st, 2)):
        st.i += 1
        return True
    return False


def solve(s):
    st = St(s)
    return expression(st, 0)

def main():
    ans = []
    while True:
        s = input()
        if (s=="."):
            break
        ans.append(solve(s))
    for i in ans:
        print("yes" if i else "no")


main()