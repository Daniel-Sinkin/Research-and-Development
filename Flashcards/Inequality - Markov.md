State and prove the Markov inequality.
&*
Let $X$ be a non-negative random variable and suppose that $\mu := E(X)$ exists. For any $t > 0$ we have
$$
P(X > t) \leq \frac{\mu}{t}.
$$
$$
\begin{aligned}
\mu &= \int_0^\infty x f(x) dx \\
&= \int_0^t xf(x) dx + \int_t^\infty xf(x) \\
&\geq \int_t^\infty xf(x) dx \\
&\geq t \int_t^\infty f(x) dx \\
&= tP(X > t).
\end{aligned}
$$
<!--SR:!2023-05-09,2,150-->