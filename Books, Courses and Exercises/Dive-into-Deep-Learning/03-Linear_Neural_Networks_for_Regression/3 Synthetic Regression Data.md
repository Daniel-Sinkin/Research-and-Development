The following class is used to define our synthetic data for the regression.
```python
class SyntheticRegressionData(d2l.DataModule):
    def __init__(self, w, b, noise=0.01, num_train=1000, num_val=1000,
                 batch_size=32):
        super().__init__()
        self.save_hyperparameters()
        n = num_train + num_val
        self.X = tf.random.normal((n, w.shape[0]))
        noise = tf.random.normal((n, 1)) * noise
        self.y = tf.matmul(self.X, tf.reshape(w, (-1, 1))) + b + noise
```
The `super().__init__()` calls the initialization function of the *parent class*, that is, of `d2l.DataModule`. Afterwards it saves all the local variables as hyperparameters as before. The command `w.shape[0]` gives the dimension $d$ of $w \in \mathbb{R}^d$. Note that `noise` is input as a (small positive) float value and then overwritten to be the normally distributed noise, scaled by that number. With the given `self.X`, `w`, `b` and the just calculated `noise` we can define the values of `self.y` by
```python
self.y = tf.matmul(self.X, tf.reshape(w, (-1, 1))) + b + noise
```
Where this is just $y = Xw + b$ with some additional normally distributed noise, i.e.,
$$
\begin{aligned}
\hat{y} = Xw + b + \eta \cdot \varepsilon,&& \varepsilon \sim \mathcal{N}(0, 1).
\end{aligned}
$$
The command `tf.reshape(w, (-1, 1))` basically transposes `w`. Note that the `-1` in this case is just shorthand for `w.shape[0]`.It turns a $1 \times d$ vector into $d \times 1$ vector.

Using this class we can then create synthatic data by fixing some $w$ and $b$ and writing
```python
data = SyntheticRegressionData(w=tf.constant([2, -3.4]), b=4.2)
```