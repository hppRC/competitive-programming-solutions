N = input()
print('Yes' if (int(N)%sum(list(map(int, list(N))))==0) else 'No')