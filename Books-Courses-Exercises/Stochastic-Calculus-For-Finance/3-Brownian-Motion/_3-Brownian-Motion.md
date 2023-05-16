SLater on we will define a type of Brownian motion as a scaled limit of a symmetric random walk, so let's start by discussing what this means.

Let $X_i, i \in \mathbb{N}_0$ be a sequence of random variables such that $X_i \in \{-1, 1\}$ with equal probability and consider the symmetric random walk
$$
Y_n := \sum_{i = 0}^n X_i.
$$
It is called symmetric because $P(X = +1) = P(X = -1)$. Let $n \in \mathbb{N}$ be arbitrary and let $0 = k_0 < k_1 < \dots < k_m = n$ be some sequence then we can write
$$
Y_n = \sum_{i = 0}^n X_i = \sum_{j = 0}^{m - 1} \sum_{i = k_j + 1}^{k_{j + 1}} X_i,
$$
because the $k_j$ define a partition of $\{0, 1, \dots, n\}$. Furthermore
$$
\sum_{i = a}^b X_i = Y_b - Y_{a - 1},
$$
as such
$$
Y_n = \sum_{j = 0}^{m - 1} Y_{k_{j + 1}} - Y_{k_j}.
$$
We call these $Y_{k_j + 1} - Y_{k_j}$ an increment of the random walk. Increments are independent of each other, have expected value equal to $0$ and a variance of $k_{j + 1} - k_j$.

Let $\mathcal{F}_k$ be a suitable filtration. Recall that we use this to model the information gained after knowing the first $X_i$ values for $1 \leq i \leq k$. Given the properties from before we would assume that our symmetric random walk is a Martingale, which turns out to be the case as can be seen in the following computation:
$$
\begin{aligned}
	E[M_\ell|\mathcal{F}_k] &= E[(M_\ell - M_k) + M_k | \mathcal{F}_k] \\
	&\overset{1}{=} E[M_\ell - M_k|\mathcal{F}_k] + E[M_k|\mathcal{F}_k] \\
	&\overset{2}{=} E[M_\ell - M_k|\mathcal{F}_k] + M_k \\
	&\overset{3}{=} E[M_\ell - M_k] + M_k = M_k,
\end{aligned}
$$
where we have used the linearity of (the induced) expected value in (1), that the value of $M_k$ only depends on the first $k$ coinflips in (2), and the independence in (3).

The scaled symmetric random walk then can be written as
$$
W^{(n)}(t) = \frac{1}{\sqrt{n}} Y_{nt}.
$$
We obtain a Brownian motion from the limit as $n \rightarrow \infty$ of $W^{(n)}$ and want to prove the following theorem now.
## 3.2.1 Central Limit Theorem
Fix $t \geq 0$. As $n \rightarrow \infty$, the distribution of the scaled random walk $W^{(n)}(t)$ evaluated at time $t$ converges to the normal distribution with mean zero and variance t.

### Proof
Recall that the PDF of such a normal distribution is given by
$$
f(x) = \frac{1}{\sqrt{2\pi t}}e^{-\frac{x^2}{2t}}.
$$

The moment generating function is given by
$$
\begin{aligned}
\varphi(u) &= \int_{-\infty}^\infty \exp\left(ux - \frac{x^2}{2t} \right) dx\\
&= e^{\frac{1}{2} u^2 t} \frac{1}{\sqrt{2\pi t}} \int_{-\infty}^\infty
 \exp \left(-\frac{(x - ut)^2}{2t}\right)dx \\
 &= e^{\frac{1}{2}u^2t},
\end{aligned}
$$
where in the last step we have used the fact that the normalized integral expression is the PDF for a normal distribution with mean $ut$ and variance $t$, and therefore that integral evaluates to $1$.

The next step is to compute the moment generating function $\varphi_n$ for $W^{(n)}$ and show that $\varphi_n \rightarrow \varphi$ pointwise. Let $t$ be such that $nt \in \mathbb{Z}$ then
$$
\varphi_n(u) = Ee^{uW^{(n)}(t)} = E \exp\left(\frac{u}{\sqrt{n}} M_{nt}\right) = \dots
$$

The rest is just a calculation.

## Log-Normal Distribution as the Limit of the Binomial Model
Assume an interest rate of $r = 0$, up factor of $u_n = 1 + \frac{\sigma}{\sqrt{n}}$ and a down factor of $d_n = 1 - \frac{\sigma}{\sqrt{n}}$. The risk neutral probabilities are then given by
$$
\tilde{p} = \frac{(1 + r) - d_n}{u_n - d_n} = \frac{1}{2}
$$
and
$$
\tilde{q} = \frac{u_n - (1 + r)}{u_n - d_n} = \frac{1}{2}.
$$
Our stockprice is given as a count of heads minus a count of tails:
$$
Y_{nt} = H_{nt} - T_{nt}
$$
and the total number of coin flips is
$$
nt = H_{nt} + T_{nt}.
$$

The stock price of time $t$ with those up and down factors is given by
$$
S_n(t) = S(0)u_n^{H_{nt}} d_n^{T_{nt}}.
$$

It can be shown that the following holds:
### Theorem 3.2.2
As $n \rightarrow \infty$, the distribution $S_n(t)$ converges to the distribution of
$$
	S(t) = S(0) \exp\left(\sigma W(t) - \frac{1}{2} \sigma^2 t\right)
$$
where $W(t)$ is a normal random variable with mean zero and variance $t$.

Such a distribution is called log-normal.