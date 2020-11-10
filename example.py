from prime_num import mypackage
import cProfile, pstats, io

NUM_OF_TRIALS = 10000000

def profile(func):
    def _func(*args, **kwargs):
        pr = cProfile.Profile()
        pr.enable()
        print(f"\n<<<     {func}     >>>")
        res = func(*args, **kwargs)
        p = pstats.Stats(pr)
        p.strip_dirs().sort_stats('cumtime').print_stats(20)
        print(f"\n<<<     {func}     >>>")
        return res
    return _func


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    elif n == 2 or n == 3 or n == 5:
        return True
    elif n % 2 == 0 or n % 3 == 0 or n % 5 == 0:
        return False
    k = 3
    while k*k <= n:
        if n % k == 0:
            return False
        k += 2
    return True


@profile
def example1():
    for i in range(NUM_OF_TRIALS):
        if mypackage.is_prime(i):
            pass


@profile
def example2():
    for i in range(NUM_OF_TRIALS):
        if is_prime(i):
            pass


@profile
def example3():
    mypackage.example3()


def main():
    #example1()
    #example2()
    example3()
    

if __name__ == "__main__":
    main()