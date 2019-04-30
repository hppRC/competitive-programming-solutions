N = int(input())
ans = ''

while (N!=0) :
    if (N%2) :
        ans = '1' + ans
        N -= 1
    else :
        ans = '0' + ans
    N /= -2

print(ans if (ans != '') else '0')




#奇数-負数
#偶数-正数


