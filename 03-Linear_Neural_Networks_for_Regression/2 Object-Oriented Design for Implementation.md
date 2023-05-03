In this section they introduce the rough structure of their API, which will be used in the rest of the book. It is quite complicated to understand, and I don't think they thoroughly explain it, treating it more as a black box. I'll try to get some grasp on it, but won't spend too much time on it, I can always learn it more deeply later on. The exercises for this section are exactly this, looking into their code.

## HyperParameters
We begin by trying to understand the `Hyperparameters` class:
```python
class HyperParameters:
    def save_hyperparameters(self, ignore=[]):
        frame = inspect.currentframe().f_back
        _, _, _, local_vars = inspect.getargvalues(frame)
        self.hparams = {k:v for k, v in local_vars.items()
                        if k not in set(ignore+['self']) and not k.startswith('_')}
        for k, v in self.hparams.items():
            setattr(self, k, v)
```

The input variable `ignore` culls a collection of variables which we don't want to save. `frame` gets the frame of the function which called `save_hyperparameters`. If a function `B` calls `save_hyperparameters` we'll want to save the local variables of `B` with this function.

The `inspect.currentframe()` is a stack which consists of the chained function calls. We use `.f_back` to get one call back, which is precisely the function which has just called `save_hyperparameters`, i.e. `B`. As such the variable `frame` is a reference to `B`. We can extract the local variables of `B` by using
```python
_, _, _, local_vars = inspect.getargvalues(frame)
```
The function `.getargvalues` returns a tuple `ArgInfo(args, varargs, keywords, locals)`, of which we are only interested in `locals`. The `_` are placeholders and indicated that we are not interested in those particular values.

Alright, so now we got a reference to `B` by considering the previous call on the function stack, saving this reference to `frame` and extracted the corresponding local variables into `local_vars`.

```python
self.hparams = {k:v for k, v in local_vars.items()
                        if k not in set(ignore+['self']) and not k.startswith('_')}
```

This uses *List Comprehension* to save all the key-value pairs as a dictionary, filtering out all variables which are in the `ignore` input variable or which start with `_`. The keys are, of course, the variable names and the corresponding values are the values of the variables of `B`. Note that `.hparams` is just some variable name, we could have called it something else.

What remains is to save those parameters in `self` to be able to access them later:
```python
for k, v in self.hparams.items():
	setattr(self, k, v)
```

Here `setattr` instantiates the attribute `k` such that `self.k == v`.

