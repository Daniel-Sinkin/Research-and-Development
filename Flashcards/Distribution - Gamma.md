Define the Gamma function, the PDF of a Gamma distributed r.v., mean, variance, summation formula. How is the exponential distribution related to the Gamma distribution?
&*
Let $\alpha > 0$, the Gamma function is
$
\Gamma(\alpha) = \int_0^\infty y^{\alpha - 1} e^{-y} dy.
$
We say $X \sim \operatorname{Gamma}(\alpha, \beta)$ if it has the PDF
$
f(x) = \frac{1}{\beta^\alpha\Gamma(\alpha)}x^{\alpha - 1} e^{-x / \beta},
$
for $x > 0$, where $\alpha, \beta > 0$.
mean = $\alpha\beta$, variance = $\alpha\beta^2$.
$X_i \sim \operatorname{Gamma}(\alpha_i, \beta)$ independent then
$
\sum_i X_i \sim \operatorname{Gamma}\left(\sum_i \alpha_i, \beta\right).
$