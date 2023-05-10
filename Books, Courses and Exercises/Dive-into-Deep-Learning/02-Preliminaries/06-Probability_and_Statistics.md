y# 1 A Simple Example: Tossing Coins
`tfd.Multinomial` takes in a number of samples, a finite set of probabilities and then gives a sampling corresponding to the number of samples. For example if `num_samples` = $100$ and `probabilities` = $[0.5, 0.5]$ then we are simply flipping a fair coin 100 times.

```py
my_array = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
my_array = my_array / sum(my_array)
my_tensor = tf.convert_to_tensor(my_array, dtype=tf.float32)
tfd.Multinomial(1000, my_tensor).sample()
```

Important concepts is the *Law of Large Numbers* and the *Central Limit Theorem*.

# 2 A More Formal Treatment
Let $(\Omega, \mathcal{A}, P)$ be a probability space, formaly this means that $\Omega$ is some set, $\mathcal{A}$ is a $\sigma$-algebra on $\Omega$ and $P$ is a measure on $(\Omega, \mathcal{A})$ such that $P(\Omega) = 1$. We call $\omega \in \Omega$ a sample and $\alpha \in \mathcal{A}$ an event.

# 3 Random Variable
A random variable is a measureable function
$$
X : \Omega \rightarrow \mathbb{R}.
$$
Recall that a numeric function is a measureable function $X : \Omega \rightarrow \overline{\mathbb{R}}$ where $\overline{\mathbb{R}} = \mathbb{R} \cup \{\pm \infty\}$. I guess we could also define this as our random variables,  but I'll stick to less general definition from before unless it comes up in the course.

Regarding notation, so because $X$ is a measureable function $X : (\Omega, \mathcal{A}) \rightarrow \mathbb{R}$ it follows that for all measureable subsets $A \subseteq \mathbb{R}$ the set
$$
X^{-1}(A) = \{\omega \in \Omega : X(\omega) \in A\}
$$
is measureable in $(\Omega, \mathcal{A})$, i.e., $X^{-1}(A) \in \mathcal{A}$. Fix $\alpha \in \mathbb{R}$ then $\{\alpha\}$ is measureable ($\{\alpha\}^c = (-\infty, \alpha) \cup (\alpha, +\infty)$ is measureable because it is open as the union of two open sets). We write
$$
\begin{aligned}
P(X^{-1}(\alpha)) &= P(\{\omega \in \Omega : X(\omega) = \alpha\}) \\
&= P(X = \alpha),
\end{aligned}
$$
or if no disambiguity exists then simply $P(\alpha)$. Consider the joint probability:
$$
\begin{aligned}
P(X = \alpha, Y = \beta) &= P(\{\omega \in \Omega : X(\omega) = \alpha\} \cap \{\omega \in \Omega : Y(\omega) = \beta\}) \\
&= P(\{\omega \in \Omega : X(\omega) = \alpha \wedge Y(\omega) = \beta\}).
\end{aligned}
$$
This definition can be extended to an arbitrary finite (I guess could even do countably infinite) amount of random variables in the obvious way. We say that $X$ and $Y$ are independent random variables if for all $\alpha, \beta \in \mathbb{R}$ the identity
$$
P(X = \alpha, Y = \beta) = P(X = \alpha) P(Y = \beta)
$$
holds. In that case we write
$$
P(X, Y) = P(X) P(Y).
$$

We write $P(\alpha \leq X \leq \beta) := P(X^{-1}\{\omega \in \Omega : \alpha \leq X(\omega) \leq \beta\})$.

# 4 Multiple Random Variables
Note that
$$
P(X = \alpha) = \sum_\beta P(X = \alpha, Y = \beta)
$$
and that
$$
\frac{P(X = \alpha, Y = \beta)}{P(X = \alpha)} \leq 1.
$$
We call the latter expression the *conditional probability*, denoting it by
$$
P(Y = \beta | X = \alpha).
$$
This is read as, the probability that $Y = \beta$ under the assumption that $X = \alpha$. In view of the sum property from before the expression $P(- | X = \alpha)$ is itself a probability function. With a slight abuse of notation we can write conditional probabilities in form of *Bayes' theorem*:
$$
P(A | B) = \frac{P(B|A) P(A)}{P(B)}.
$$
In Bayesian statistics we begin with a a-priori hypothesis $P(H)$ and a *likelihood function* which tells us that if we assume the hypothesis to be true then how likely is $P(E|H)$. We can update our probability to produce the *a-posteriori* belief
$$
P(H|E) = \frac{P(E|H)P(H)}{P(E)}.
$$
Using the notation of conditional probability we can express independence of random variables as
$$
P(X|Y) = P(X)
$$
which considering Bayes' theorem is equivalent to $P(X, Y) = P(X)P(Y)$.

> Interestingly, two variables can be independent in general but become dependent when conditioning on a third. This often occurs when the two random variables $X$ and $Y$ correspond to causes of some third variable $C$. For example, broken bones and lung cancer might be independent in the general population but if we condition on being in the hospital then we might find that broken bones are negatively correlated with lung cancer. That is because the broken bone _explains away_ why some person is in the hospital and thus lowers the probability that they have lung cancer.

# 5 An Example

# 6 Expectation
The *expectation* (or average) of a random variable $X$ is defined by
$$
E[X] = E_{x \sim P}[x] = \sum_x x P(X = x)
$$
or if we're talking about densities
$$
E[X] = \int x p(x) dx.
$$
The expected value of some function $f$ is given by
$$
E_{x \sim P}[f(x)] = \sum_x f(x) P(x)
$$
or if we're talking about densities
$$
E_{x \sim P}[f(x)] = \int f(x) p(x) dx.
$$
The *variance* is given by
$$
\operatorname{Var}[X] E[(X - E[X])^2] = E[X^2] - E[X]^2
$$
where, of course, $E[X^2] = E[f(X)]$ for $f(x) = x^2$ as given before. The square root of  the variance is called the *standard deviation*. Similiarly to before the variance of a random variable is defined by
$$
\operatorname{Var}_{x \sim P}[f(x)] = E_{x \sim P}[f^2(x)] - E_{x \sim P}[f(x)]^2.
$$
Note that we often denote the mean by $\mu$ and the variance by $\sigma^2$ (and of course the standard deviation by $\sigma$).

In higher dimensions we can extend the expectation linearly to vector valued functions. The higher-dimensional analogue of the variance is given by the quadratic form of the so-called *covariance matrix*
$$
\Sigma := \operatorname{Cov}_{x \sim P}[x] = E_{x \sim P}[(x - \mu)(x - \mu)^t].
$$
Based on this we define the variance of a linear function $x \mapsto v^tx$ by
$$
v^t \Sigma v = E_{x \sim P}[v^t(x - \mu)(x - \mu)^tv] = \operatorname{Var}_{x \sim P}[v^t x].
$$
# 7 Discussion
Recall *Chebyshev's inequality* which states that
$$
P(|X - \mu| \geq k \sigma) \leq \frac{1}{k²}
$$