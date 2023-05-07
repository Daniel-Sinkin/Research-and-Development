State and prove Chebyshev's inequality using Markov's inequality.
&
Let $\mu := E(X)$ and $\sigma^2 = V(X)$, then
$$
P(|X - \mu| \geq t) \leq \frac{\sigma^2}{t^2}
$$
and
$$
P(|Z| \geq k) \leq \frac{1}{k^2}
$$
where $\frac{Z - \mu}{\sigma}$.
Proof:
$$
\begin{aligned}
P(|X - \mu| \geq t) &= P(|X - \mu|^2 \geq t^2) \\
&\leq \frac{E(X - \mu)^2}{t^2} \\
&= \frac{\sigma^2}{t^2}.
\end{aligned}
$$
The second part follows by setting $t = k\sigma$.
<!--SR:!2023-05-09,2,170-->