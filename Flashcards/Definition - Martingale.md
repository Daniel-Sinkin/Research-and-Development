Define what a (sub/super)martingale is. 
&*
Let $(\Omega, \mathcal{F}, P)$ be a probability space, let $T$ be a fixed positive number, and let $\mathcal{F}(t), 0 \leq t \leq T,$ be a filtration of sub-$\sigma$-algebras of $\mathcal{F}$. Consider an adapted stochastic process $M(t), 0 \leq t \leq T$.
1. If $$E[M(t)|\mathcal{F}(s)] = M(s)$$ for all $0 \leq s \leq t \leq T$, we say this process is a martingale. It has no tendency to rise or fall.
2. If $\geq$ (resp. $\leq$) in the first case holds, then we call process $M$ a submartingale (resp. supermartingale). It has no tendency to fall (resp. rise); it may have a tendency to rise (reps. fall).
The condition can basically be stated that if we have full information until time $s$, then we expect the value of $M(t), t > s$ to be equal to the value of $s$, i.e., there is no trend to the data. 
<!--SR:!2023-05-09,2,150-->