* Vocabulary
	* *Training Dataset* or *training set*
	* *example* or *data point*, *instance* or *sample*
	* *label* or *target*
	* *features* or *covariates*
	* *weights*
	* *biases* or *offset* or *intercept*
	* *design matrix*
	* *loss function*
	* *fitness*
	* *gradient descent*
	* *stochastic gradient descent* or SGD
	* *minibatch stochastic gradient descent* or MB-SGD
	* *hyperparameters*
	* *validation dataset* or *validation set*
	* *generalization*
	* *inference* or *prediction*
	* *likelihood*
	* *principle of maximum likelihood*
	* *negative log-likelihood*

# 1 Basics

The setup is the following, the conditional mean $E[Y | X = x]$ can be expressed as a affine linear transformation of the features $x$, i.e. if $x = (x^{(1)}, \dots, x^{(n)})$ then
$$
E[Y = y| X = x] = \sum_{i = 1}^n w^i(x, y) x^{(i)} + b.
$$
Furthermore we assume there is some kind of observational noise, and that this noise is well-behaved in the sense that it follows a Guassian distribution.

## 1.1 Model
We can compactly express our affine linear transformation
$$
\hat{y} = w^t x + b.
$$
When we have multidimensional feature sets we can combine them into one so-called *feature matrix*, with which the setup then becomes
$$
\hat{y} = Xw + b.
$$

## 1.2 Loss Function
We need a *loss function* to measure the quality of our weight so we can know if a new choice of weights is preferrable. They measure the *fitness*. The squared error loss function is given by
$$
l^{(i)}(w, b) = \frac{1}{2}(\hat{y}^{(i)} - y^{(i)})^2.
$$
The total quality of our model is then the average (or equivalently the sum) of the losses on our training set:
$$
L(w, b) = \overline{l(w, b)}
$$
where
$$
l(w, b)_i = l^{(i)}(w, b)
$$
and $\overline{v}$ denotes the mean. Written out this can be expressed as
$$
L(w, b) = \frac{1}{n} \sum_{i = 1}^n \frac{1}{2} \left( w^t x^{(i)} + b - y^{(i)}\right)^2.
$$
With this we can formulate our problem as the following optimization problem:
$$
w^*, b^* = \arg\min_{w, b} L(w, b).
$$

## 1.3. Analytic Solution
We can incorporate the bias into our system by adding a column of ones to $X$, i.e.
$$
X \in \mathbb{R}^{m \times n} \mapsto \left(\begin{array}{cccc|c}
  x_1^{(1)} & x_1^{(2)} & \cdots & x_1^{(n)} & 1 \\
  x_2^{(1)} & x_2^{(2)} & \cdots & x_2^{(n)} & 1 \\
  \vdots    & \vdots    & \ddots & \vdots    & \vdots \\
  x_m^{(1)} & x_m^{(2)} & \cdots & x_m^{(n)} & 1
\end{array}\right) \in \mathbb{R}^{m \times (n + 1)}.
$$
We call this new matrix also $X$. We want to minimize $||y - Xw||^2$. Assuming that $X$ has full rank then there will only be a single critical point. Differentiating the new loss function yields
$$
\partial_w||y - Xw||^2 = 2 X^t(Xw - y) \overset{!}{=} 0.
$$
This is equivalent to $X^ty = X^tXw$. Suppose $X^tX$ is invertible (this is the case if $m \geq n + 1$ and $X$ has full rank), then we can solve for $w$:
$$
w = (X^tX)^{-1}X^ty.
$$

## 1.4 Minibatch Stochastic Gradient Descent
Here we learn about the celebrated *gradient descent* algorithm. One way to alleviate the common problems of GD is to use *stochastic gradient descent* (SGD). For us a combination of (deterministic) GD and SGD, which instead of picking a single parameter to update takes random batches, usually between $32$ and $256$.

Fix a batch size $n_b$, MB-SGD then works by picking a random subset $B$ of the training set such that $|B| = n_b$ and applies the following update on the weights,
$$
(w, b) \leftarrow (w, b) - \frac{\eta}{n_b} \sum_{i \in B} \partial_{(w, b)} l^{(i)}(w, b).
$$
we call $\eta > 0$ the learning rate. We call parameters like $n_b$ and $\eta$, which are tuneable parameters which don't change during the training loop, *hyperparameters*. We typically then check the quality of the solution on a separate *validation dataset*. Instead of finding global minimizers on the training we are more interested in *generalization*, i.e. being apply to predict things which are not in our training set.

## 1.5 Predictions
In the literature predictions are often called *inference*. This book will try to stick to prediction whenever possible.

# 2 Vectorization for speed

# 3 The normal distribution and Squared Loos
Recall that we made the assumption that our observational noise is normally distributed, more accurately we assume:
$$
y = w^tx + b + \varepsilon
$$
where $\varepsilon \sim \mathcal{N}(0, \sigma^2)$. The *likelihood* of seeing a particular $y$ for a given $x$ can be expressed as
$$
P(y | x) = \frac{1}{\sqrt{2 \pi \sigma^2}} \exp\left( - \frac{1}{2\sigma^2}(y - w^t x - b)^2 \right).
$$
According to tthe *principle of maximum likelihood*, the best values of parameters $w$ and $b$ are those that maximize the likelihood of the entire dataset:
$$
P(y|X) = \prod_{i = 1}^n p(y^{(i)}|x).
$$
This equality holds because we have drawn the samples $(x^{(i)}, y^{(i)})$ indepedently of each other. Estimators chosen according to the principle of maximum likelihood are called *maximum likelihood estimators*. For historical reasons we want to reformulate this as a minimization problem. Applying the log this turns into a sum, adding a $-$ turns it into a minimization problem, we want to minimize the *negative log-likelihood*:
$$
-\log P(y|X) = \sum_{i = 1}^n \frac{1}{2} \log(2 \pi \sigma^2) + \frac{1}{2\sigma^2} \left( y^{(i)} - w^tx^{(i)} - b \right)^2
$$
Because the first sum doesn't depend on $w, b$ and the solution is independent of $\sigma$ this is equivalent to minimizing the squared error. This can be stated as follows:
> It follows that minimizing the mean squared error is equivalent to maximum likelihood estimation of a linear model under the assumption of additive Gaussian noise.

# 4 Linear Regression as a Neural Network
We can think of linear regression as a single-layer fully connected neural network.