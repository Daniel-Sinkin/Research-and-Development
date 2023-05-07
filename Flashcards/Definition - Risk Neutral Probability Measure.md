Define the risk-neutral probability measure. State the associated theorem and prove it. Explain how it is related to [[Definition - Radon-Nikodym Derivative]]. Discuss the induced expected value.
&*
Let $(\Omega, \mathcal{F}, P)$ be a probability space and let $Z$ be an almost surely nonnegative random variable with $EZ = 1$. For $A \in \mathcal{F}$, define
$
\tilde{P}(A) = \int_A Z(\omega)dP(\omega.)
$
Then $\tilde{P}$ is a probability measure. Furthermore, if $X$ is a nonnegative random variable, then
$
\tilde{E}X = E[XZ].
$
If $Z$ is almost surely strictly positive, we also have
$
E[Y] = \tilde{E}\left[\frac{Y}{Z}\right]
$
for every nonnegative random variable Y. 
<!--SR:!2023-05-09,2,150-->

Theorem 1.6.1. in [[_Stochastic-Calculus-For-Finance]].