fx = lambda x: int(x/2) if x%2==0 else 3*x + 1
ans = [int(input())]
counter = 1

while True:
    new_x = fx(ans[-1])
    counter += 1
    if (new_x in ans) :
        break
    ans.append(new_x)

print(counter)