Define the Radon-Nikodym derivative. Discuss the Radon-Nikodym theorem.
&*
Let $(\Omega, \mathcal{F}, P)$ be a probability space, let $\tilde{P}$ be another probability measure on $(\Omega, \mathcal{F})$ that is equivalent to $P$, and let $Z$ be an almost surely positive random variable that relates $P$ and $\tilde{P}$ via
$
\tilde{P}(A) = \int_A Z(\omega)dP(\omega).
$
Then $Z$ is called the Radon-Nikodym derivative of $\tilde{P}$ with respect to $P$, and we write
$
Z = \frac{d\tilde{P}}{dP}.
$
It turns out that given two equivalent probability measures then a Radon-Nikodym exists for those, in particular this means the form above is the only way equivalent prob. measures can look.
<!--SR:!2023-05-09,2,150-->

Definition 1.6.5. and Theorem 1.6.7. in [[_Stochastic-Calculus-For-Finance]].