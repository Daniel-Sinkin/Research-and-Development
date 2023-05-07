Define independence for $\sigma$-algebras, random variables.
&*
Let $(\Omega, \mathcal{F}, P)$ be a probability space, and let $\mathcal{G}$ and $\mathcal{H}$ be sub-$\sigma$-algebras of $\mathcal{F}$ (i.e. the sets in $\mathcal{G}$ and the sets in $\mathcal{H}$ are also in $\mathcal{F}$). We say these two $\sigma$-algebras are independent if
$$
P(A \cap B) = P(A)P(B
$$
for all $A \in \mathcal{G}$ and $B \in \mathcal{H}$. Let $X$ and $Y$ be random variables on $(\Omega, \mathcal{F}, P)$. We say these two random variables are independent if the $\sigma$-algebras they generate, $\sigma(X)$ and $\sigma(Y)$, are independent. We say that the random variable $X$ is independent of the $\sigma$-algebra $\mathcal{G}$ if $\sigma(X)$ and $\mathcal{G}$ are independent.
<!--SR:!2023-05-09,2,150-->

Definition 2.2.1 in [[_Stochastic-Calculus-For-Finance]]