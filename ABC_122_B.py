S = input()

subStringList = [S]
length = len(S)

counter = 0
before = 1

while (not(counter == before)):
    before = counter
    for s in subStringList[:]:
        for letter in s:
            if (not(letter in "ATGC")):
                subStringList += s.split(letter)
                subStringList.remove(s)
                counter = len(subStringList)
                break

print(len(max(subStringList,key=len)))
