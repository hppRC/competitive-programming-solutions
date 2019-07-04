A, B, C, D = map(int, input().split())

print('Left' if (A+B > C+D) else 'Balanced' if (A+B==C+D) else 'Right')