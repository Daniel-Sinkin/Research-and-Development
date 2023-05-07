Define what a Markov Process is. Also give the expression via time-parametrized function.
&*
Let $(\Omega, \mathcal{F}, P)$ be a probability space, let $T$ be a fixed positive number, and let $\mathcal{F}(t), 0 \leq t \leq T,$ be a filtration of sub-$\sigma$-algebras of $\mathcal{F}$. Consider an adapted stochastic process $X(t), 0 \leq t \leq T$. Assume that for all $0 \leq s \leq t \leq T$ and for every nonnegative, Borel-measurable function $f$, there is another Borel-measureable function $g$ such that
$
E[f(X(t))|\mathcal{F}(s)] = g(X(s)).
$
Then we say that $X$ is a Markov process.
Note that $g = g_s$ and we can write $g_t(x) = f(t, x)$, then the expression above becomes
$
E[f(t, X(t))|\mathcal{F}(s)] = f(s, X(s))
$
for $0 \leq s \leq t \leq T$.
<!--SR:!2023-05-09,2,150-->