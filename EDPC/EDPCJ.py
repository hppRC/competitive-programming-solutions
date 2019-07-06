import sys

def I(): return int(sys.stdin.readline())
def LI(): return list(map(int, sys.stdin.readline().split()))

#実家DPと気合い遷移式

def main():
    N = I()
    a = LI()
    t = [0, 0, 0]
    for ai in a:
        t[ai-1] += 1
    A, B, C = t
    E = B+C

    dp = [[0 if (not i|j) else N/(i+j) for j in range(E+1)] for i in range(N+1)]

    for k in range(C+1):
        for j in range(E+1-k):
            for i in range(N+1-k-j):
                z = i+j+k
                if (not z):
                    continue
                d = dp[i][j]
                if (i<N):
                    dp[i+1][j] += (i+1)*d/(z+1)
                if (i>0)&(j<E):
                    dp[i-1][j+1] += (j+1)*d/z
                if (j>0)&(k<C):
                    dp[i][j-1] = (k+1)*d/z + N/z

    print(dp[A][B])

if __name__ == "__main__":
    main()



"""
def main():
    N = I()
    A = LI()
    t = [0, 0, 0]
    for a in A:
        t[a-1] += 1
    A, B, C = t

    dp = [[[0]*(N+1) for i in range(N+1)] for j in range(N+1)]

    for k in range(N+1):
        for j in range(N+1):
            for i in range(N+1):
                if (i==0) and (j==0) and (k==0):
                    continue
                dp[i][j][k] += N/(i+j+k)
                d = dp[i][j][k]
                if (i<N):
                    dp[i+1][j][k] += (i+1)*d/(i+1+j+k)
                if (i>0) and (j<N):
                    dp[i-1][j+1][k] += (j+1)*d/(i+j+k)
                if (j>0) and (k<N):
                    dp[i][j-1][k+1] += (k+1)*d/(i+j+k)

    print(dp[A][B][C])

main()

"""

"""
def tail_recursion_with_stack_inspection(g):
    '''
    Version of tail_recursion decorator using stack-frame inspection.    
    '''
    loc_vars ={"in_loop":False,"cnt":0}
    
    def result(*args, **kwd):
        if not loc_vars["in_loop"]:
            loc_vars["in_loop"] = True
            while 1:            
                tc = g(*args,**kwd)
                try:                    
                    qual, args, kwd = tc
                    if qual == 'continue':
                        continue
                except TypeError:                    
                    loc_vars["in_loop"] = False
                    return tc                                    
        else:
            f = sys._getframe()
            if f.f_back and f.f_back.f_back and \
                  f.f_back.f_back.f_code == f.f_code:
                return ('continue',args, kwd)
            return g(*args,**kwd)
    return result


def tail_recursion(g):
    '''
    Version of tail_recursion decorator using no stack-frame inspection.    
    '''
    loc_vars ={"in_loop":False,"cnt":0}

    def result(*args, **kwd):
        loc_vars["cnt"]+=1
        if not loc_vars["in_loop"]:
            loc_vars["in_loop"] = True
            while 1:            
                tc = g(*args,**kwd)
                try:                    
                    qual, args, kwd = tc
                    if qual == 'continue':
                        continue
                except (TypeError, ValueError):                    
                    loc_vars["in_loop"] = False
                    return tc                                    
        else:
            if loc_vars["cnt"]%2==0:
                return ('continue',args, kwd)
            else:
                return g(*args,**kwd)
    return result


N = I()
a = LI()

"""
"""
dp = [[0]*(N+1) for i in range(N+1)]


@lru_cache(maxsize=None)
def rec(i, j, k):
    if (i==0) and (j==0) and (k==0):
        return 0
    res = N
    if (i>0):
        print(rec(i-1, j, k)*i)
        res += rec(i-1, j, k)*i
    if (j>0):
        res += rec(i+1, j-1, k)*j
    if (k>0):
        res += rec(i, j+1, k-1)*k
    res /= i+j+k
    return res
"""

"""
dp = [[[-1]*(N+1) for i in range(N+1)] for j in range(N+1)]
 
def rec(i, j, k):
    if (dp[i][j][k] > -1):
        return dp[i][j][k]
    if (i==0) and (j==0) and (k==0):
        return 0
    res = N
    if (i>0):
        res += rec(i-1, j, k)*i
    if (j>0):
        res += rec(i+1, j-1, k)*j
    if (k>0):
        res += rec(i, j+1, k-1)*k
    res /= i+j+k
    dp[i][j][k] = res
    return res
 
print(rec(a.count(1), a.count(2), a.count(3)))
"""


"""
dp = [[[0]*(N+1) for i in range(N+1)] for j in range(N+1)]
dp[1][0][0] = 1/N
for i in range(N+1):
    for j in range(N+1):
        for k in range(N+1):
            if (not i) and (not j) and (not k):
                continue
            res = N
            if (i>0):
                res += i*dp[i-1][j][k]
            if (j>0) and (i<N):
                res += j*dp[i+1][j-1][k]
            if (k>0) and (j<N):
                res += k*dp[i][j+1][k-1]
            dp[i][j][k] = res/(i+j+k)

print(dp[a.count(1)][a.count(2)][a.count(3)])

print(rec(a.count(1), a.count(2), a.count(3)))
"""