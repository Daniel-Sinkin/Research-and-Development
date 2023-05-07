Define the normal distribution, what is the standard normal distribution? What is the mean, variance?
Define the State the additivity properties.
&
We say $X \sim \mathcal{N}(\mu, \sigma^2)$ if it has the PDF
$
f(x) = \frac{1}{\sigma\sqrt{2 \pi}} \exp\left(-\frac{1}{2\sigma^2} (x - \mu)^2\right)
$
for $x \in \mathbb{R}$.
mean = $\mu$, variance = $\sigma^2$.
If $X \sim \mathcal{N}(\mu, \sigma^2)$ then $Z = \frac{X - \mu}{\sigma^2} \sim \mathcal{N}(0, 1)$. Similiarl
$$
\mu' + \sigma' Z \sim \mathcal{N}(\mu', (\sigma')^2).
$$
If $X_i \sim \mathcal{N}(\mu_i, \sigma_i^2)$ then
$$
\sum_{i} X_i \sim N\left(\sum_i \mu_i, \sum_i \sigma_i^2\right)
$$
<!--SR:!2023-05-09,2,150-->