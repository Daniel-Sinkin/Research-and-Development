Define the multivariate normal distribution. What is the mean, what is the variance.
State the transformation theorem for it.
Split $\Sigma$ into 2x2 blocks, what are the marginals, what is the cond. probability of one marginal conditioned on the other.
What is the distribution of $a^tX$ where $a$ is some vector.
Define the multivariate normal distribution. What is its mean and variance? What is the relationship between the multivariate normal and $\chi^2$?
&
$X \sim \operatorname{Multivariate Normal}(\mu, \Sigma)$ if it has PDF $f(x) = f(x; \mu, \Sigma)$
$
f(x) = (2\pi)^{- k / 2} \det(\Sigma)^{-1/2} \exp\left(-\frac{1}{2} (x - \mu)^t\Sigma^{-1}(x - \mu)\right).
$
mean = $\mu$, variance = $\Sigma$.
$X \sim \mathcal{N}(0, I)$ then $Z = \mu + \Sigma^{1 / 2} Z \sim \mathcal{N}(\mu, \Sigma)$. Conversely if $Z \sim \mathcal{N}(\mu, \Sigma)$ then
$$
X' = (\Sigma')^{-1/2}(X - \mu) \sim \mathcal{N}(0, I).
$$
Write $\Sigma$ in blocks as
$$
\Sigma = \begin{pmatrix}
\Sigma_{aa} & \Sigma_{ab} \\
\Sigma_{ba} & \Sigma_{bb}
\end{pmatrix}
$$
and $\mu = (\mu_a, \mu_b)$ in the obvious way.
Suppose $X \sim \mathcal{N}(\mu, \Sigma)$. Then $X_a \sim \mathcal{N}(\mu_a, \Sigma_{aa})$. 
The conditional distribution of $X_b$ given $X_a = a$ is given by
$$
X_b|X_a = x_a \sim \mathcal{N}(\mu', \Sigma')
$$
where
$$
\begin{aligned}
\mu' &= \mu_b + \Sigma_{ba}\Sigma_{aa}^{-1}(x_a - \mu_a) \\
\Sigma' &= \Sigma_{bb} - \Sigma_{ba}\Sigma_{aa}^{-1}\Sigma_{ab}.
\end{aligned}
$$
If $a$ is some vector then $a^tX \sim \mathcal{N}(a^t \mu, a^t\Sigma a)$.
Suppose 
$$
V := (X - \mu)^t\Sigma^{-1}(X - \mu)
$$
then $V \sim \chi_n^2$ if $\Sigma \in \mathbb{R}^{n \times n}$.
<!--SR:!2023-05-08,1,130-->