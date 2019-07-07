N = int(input())
books = [input().split(" ") for i in range(N)]
Q = int(input())
query = [input().split(" ") for i in range(Q)]
dates = []
for _, _, date in books:
    dates.append(list(map(int, date.split("/"))))


def search_Q_title(Q_title, books):
    if (Q_title=="*"):
        return (1<<(N+1))-1
    else:
        ret = 0
        for i, book in enumerate(books):
            if (Q_title in book[0]):
                ret |= 1<<i
        return ret

def search_Q_author(Q_author, books):
    if (Q_author=="*"):
        return (1<<(N+1))-1
    else:
        ret = 0
        for i, book in enumerate(books):
            if (Q_author in book[1]):
                ret |= 1<<i
        return ret

def search_Q_date_from(Q_date_from, dates):
    if (Q_date_from=="*"):
        return (1<<(N+1))-1
    else:
        ret = 0
        Q_year, Q_month, Q_day = list(map(int, Q_date_from.split("/")))
        for i, date in enumerate(dates):
            year, month, day = date
            if (year > Q_year):
                ret |= 1<<i
            elif (year == Q_year):
                if (month > Q_month):
                    ret |= 1<<i
                elif (month == Q_month):
                    if (day >= Q_day):
                        ret |= 1<<i
        return ret


def search_Q_date_t0(Q_date_to, dates):
    if (Q_date_to=="*"):
        return (1<<(N+1))-1
    else:
        ret = 0
        Q_year, Q_month, Q_day = list(map(int, Q_date_to.split("/")))
        for i, date in enumerate(dates):
            year, month, day = date
            if (year < Q_year):
                ret |= 1<<i
            elif (year == Q_year):
                if (month < Q_month):
                    ret |= 1<<i
                elif (month == Q_month):
                    if (day <= Q_day):
                        ret |= 1<<i
        return ret
        

for i, Quer in enumerate(query):
    Q_title, Q_author, Q_date_from, Q_date_to = Quer
    bit = search_Q_title(Q_title, books) 
    bit &= search_Q_author(Q_author, books) 
    bit &= search_Q_date_from(Q_date_from, dates) 
    bit &= search_Q_date_t0(Q_date_to, dates)
    
    for j in range(N):
        if (bit & 1):
            print(books[j][0])
        bit >>= 1
    if (i != Q-1):
        print()