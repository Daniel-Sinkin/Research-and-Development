Define the normal distribution, what is the standard normal distribution? What is the mean, variance? Define the State the additivity properties.
&*
$$
\begin{aligned}
\text{PDF:} \quad && f(x) &= \frac{1}{\sqrt{2\pi\sigma^2}} e^{ -\frac{(x-\mu)^2}{2\sigma^2} }, \\
\text{CDF:} \quad && F(x) &= \frac{1}{2} \left[ 1 + \text{erf} \left( \frac{x-\mu}{\sigma\sqrt{2}} \right) \right] \\
\text{Mean:} \quad && \mu(X) &= \mu \\
\text{Variance:} \quad && V(X) &= \sigma^2
\end{aligned}
$$
If $X \sim \mathcal{N}(\mu, \sigma^2)$ then $Z = \frac{X - \mu}{\sigma^2} \sim \mathcal{N}(0, 1)$. Similiarly
$$
\mu' + \sigma' Z \sim \mathcal{N}(\mu', (\sigma')^2).
$$
If $X_i \sim \mathcal{N}(\mu_i, \sigma_i^2)$ then
$$
\sum_{i} X_i \sim N\left(\sum_i \mu_i, \sum_i \sigma_i^2\right)
$$
<!--SR:!2023-05-08,1,130-->