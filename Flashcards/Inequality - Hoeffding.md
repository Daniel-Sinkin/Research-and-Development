State Hoeffding's Inequality in the general case and for the Bernoulli Case.
Explain how to get a Confidence Interval for a binomial parameter from it.
&*
Let $Y_1, \dots, Y_n$ be independent, such that $E(Y_i) = 0$ and
$
a_i \leq Y_i \leq b_i
$
for all $i$. Fix any $\varepsilon > 0$, then for all $t > 0$ the following identity holds:
$
P\left( \sum_i^n Y_i \geq \varepsilon\right) \leq e^{-t \varepsilon} \prod_{i = 1}^n \exp\left(\frac{t^2 (b_i - a_i)^2}{8}\right).
$
Now suppose $X_1, \dots, X_n \sim \operatorname{Bernoulli}.$ Then, for any $\varepsilon > 0$ we have the following special case of the Inequality above:
$
P(|\overline{X}_n - p| > \varepsilon) \leq 2 e^{-2n\varepsilon^2}
$
where
$
\overline{X}_n = \frac{1}{n} \sum_{i = 1}^n X_i
$
is the mean. To obtain a confidence interval for a binomial parameter $p$ let $0 < \alpha < 1$ be arbitrary and consider
$
\varepsilon_n = \varepsilon_n(\alpha, n) = \sqrt{\frac{1}{2n} \log\left(\frac{2}{\alpha}\right)}.
$
By Hoeffding's inequality we then have
$
P(|\overline{X}_n - p| > \varepsilon_n) \leq \alpha
$
because $2e^{-2n\varepsilon_n^2} = \alpha$. Let $C = (\overline{X}_n - \varepsilon_n, \overline{X}_n + \varepsilon_n)$. Then, 
$
P(p \notin C) = P(|\overline{X}_n - p| > \varepsilon_n) \leq \alpha.
$
We call $C$ a $1-\alpha$ confidence interval.
<!--SR:!2023-05-09,2,150-->