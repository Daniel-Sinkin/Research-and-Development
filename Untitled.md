Let $D = \{(x, y) \in \mathbb{R}^2 : x^2 + y^2 \leq 1\}$ and note that for $A \subseteq D$ measureable
$$
P(A) = \frac{\lambda^2(A)}{\lambda^2(D)} = \frac{1}{\pi} \lambda^2(A).
$$
$$
\begin{aligned}
P(X \leq x) = x,&&P(Y \leq y) = y.
\end{aligned}
$$
We can calculate
$$
\begin{aligned}
P(R \leq r) &= P\left(\sqrt{X^2 + Y^2} \leq r\right) \\
&= P(X^2 + Y^2 \leq r^2) \\
&= \int_{\{X^2 + Y^2 \leq r^2\}} dP \\
&= \frac{1}{\pi}\int_0^r \int_{0}^{\sqrt{r^2 - y^2}} 1 dx dy \\
&= \frac{1}{\pi} \int_0^r \sqrt{r^2 - y^2} dy \\
&= \frac{1}{\pi} \pi r^2 = r^2.
\end{aligned}
$$
As such $F(r) = r^2$, to obtain the PDF we differentiate: $f(r) = F'(r) = 2r$.