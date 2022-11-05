for num in [i for i in range(10000) if i % 2 != 0 and i % 5 != 0]:
    multiple = 1

    while multiple % num != 0:
        multiple = int(str(multiple) + "1")

    print(len(str(multiple)))
    print(num)
