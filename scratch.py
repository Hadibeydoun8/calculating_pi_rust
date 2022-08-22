import math
from builtins import range, print

C = 426880 * math.sqrt(10005)


def calc_n_digits_of_pi(n):
    pi = 0
    for i in range(0, n):
        m, l, x = calc_m_l_x(i)
        term = calc_term(m, l, x)
        pi += term
    print(pi)


def calc_term(m, l, x):
    return (m * l) / x


def calc_m_l_x(n):
    m = math.factorial(6 * n) / (math.factorial(3 * n) * math.pow(math.factorial(n), 3))
    l = 545140134 * n + 13591409
    x = math.pow(-262537412640768000, n)

    return m, l, x
