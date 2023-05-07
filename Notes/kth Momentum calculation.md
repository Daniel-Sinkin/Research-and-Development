In the continuous case, when you have CDF $F$ for a r.v. $X$ then the kth moment can be calculated by
$$
E[X^k] = \int x^k dF(x),
$$
if you have a underlying PMF $f$ then this becomes
$$
E[X^k] = \int x^k f(x) dx.
$$
Explicitly note that you are not raising $f$.

In the discrete case it is
$$
E[Y^k] = \sum_n n^k P(X = n) = \sum_n n^k f(n).
$$