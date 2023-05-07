Suppose $X \sim \operatorname{Bin}(n, p)$, what is its PMF, mean, variance. What is the relationship between the Binomial and the Bernoulli distribution? If $X_i \sim \operatorname{Bin}(n_i, p)$ then determine the distribution of
$$
Y = \sum_i X_i.
$$
&*
$$
f(k) = \begin{cases}
\binom{n}{k}p^k(1 - p)^{n - k},&0 \leq k \leq n\\
0,&\text{otherwise}
\end{cases}
$$
mean = $np$ and variance = $np(1 - p)$. $Y \sim \operatorname{Bin}\left(\sum_{i = 1}^n n_i,p\right)$. Note that $\operatorname{Bin}(1, p) = \operatorname{Bernoulli}(p)$.
<!--SR:!2023-05-09,2,170-->