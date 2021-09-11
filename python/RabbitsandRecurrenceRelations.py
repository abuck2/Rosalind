n = int(input("n"))
k = int(input("k"))

def rabbits(n, k):
    n1 = 1
    n2 = 1
    count = 0
    for i in range(2, n):
        nth = n1 + n2*k
        # update values
        n2 = n1
        n1 = nth
        count += 1
    print(nth)
    return nth

rabbits(n, k)
