from prime_num import mypackage


def main():
    for i in range(100):
        if mypackage.is_prime(i):
            print(i)
    


if __name__ == "__main__":
    main()