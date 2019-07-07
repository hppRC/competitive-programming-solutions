T, D = list(map(int, input().split()))
ta, tb = list(map(int, input().split()))
da, db = list(map(int, input().split()))

ans = 10000000007



for i in range(D//da+1):
    for j in range(D//db+1):
        if (i*da + j*db <= D) and (i*da + j*db >= 1):
            tmp = (ta*i*da + tb*j*db)/(i*da + j*db)
            ans = min(abs(ans), abs(T - tmp))

print(ans)
