gcd = lambda x, y: x if (y==0) else gcd(y, x%y)
lcm = lambda x, y: int(x*y/gcd(x, y))
print(lcm(8976655, 67865574))