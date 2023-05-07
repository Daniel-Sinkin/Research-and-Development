Define the multinomial distribution, what are the marginal distributions?
&
$X \sim \operatorname{multinomial}(n, p)$ if it has the PDF
$$
f(x) = \binom{n}{\prod_i x_i} \prod_i p_i^{x_i},
$$
where
$$
\binom{n}{\prod_i x_i} = \frac{n!}{\prod x_i!}.
$$
mean = $np$, variance = ...
If $X \sim \operatorname{Multinomial}(n, p)$ where $X = (X_1, \dots, X_k)$ and $p = (p_1, \dots, p_k)$. The marginal distribution of $X_j$ is $\operatorname{Bin}(n, p_j)$.
<!--SR:!2023-05-09,2,150-->