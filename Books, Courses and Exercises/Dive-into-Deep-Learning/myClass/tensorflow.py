import tensorflow as tf
import numpy as np
import inspect

import collections

from IPython import display

# Package for plotting
from matplotlib import pyplot as plt

from matplotlib_inline import backend_inline

###
# Training
###

class HyperParameters:
	# HyperParameters are params which do not change during the training time (for example learning rate)
	# We want to set the local variables as attributes of `self`
	def save_hyperparameters(self, ignore=[]):
		# Gets the stack of function calls, `.f_back` gives previous function caller on the stack,
		# this has to be the function which has called `save_hyperparameters`
		frame = inspect.currentframe().f_back
		
		# We want to get the local values of frame
		# getargvalues returns tuples of the form (args, varargs, varkw, locals)
		# we are only interested in the locals
		_, _, _, local_vars = inspect.getargvalues(frame)

		# Note `.hparams` is just a variable name, it has no deeper meaning
		# This creates a dictionary where the keys are the variable names of the local variables of frame, i.e.,
		# of the function caller. We filter those variables which are in `ignore` as well as those
		# that start with '_'
		self.hparams = {k:v for k, v in local_vars.items()
                        if k not in set(ignore+['self']) and not k.startswith('_')}
		
		# Assigns the variables to the `self` object
		for k, v in self.hparams.items():
			setattr(self, k, v)

class Module(tf.keras.Model, HyperParameters):
    def __init__(self, plot_train_per_epoch=2, plot_valid_per_epoch=1):
		# Calls the __init__ method of the parent
        super().__init__()
        self.save_hyperparameters()
        self.board = ProgressBoard()
        self.training = None

    def loss(self, y_hat, y):
		# We don't have a generic loss function, every model has its own
        raise NotImplementedError

    def forward(self, X):
        assert hasattr(self, 'net'), 'Neural network is defined'
        return self.net(X)

    def call(self, X, *args, **kwargs):
		# *args for non-keyword (positional) arguments ~> Tuple
		# **kwargs for keyword (non-position) arguments ~> Dict
        if kwargs and "training" in kwargs:
            self.training = kwargs['training']
        return self.forward(X, *args)

    def plot(self, key, value, train):
        assert hasattr(self, 'trainer'), 'Trainer is not initialized'
	
        self.board.xlabel = 'epoch'
	
        if train:
            x = self.trainer.train_batch_idx / self.trainer.num_train_batches
            n = self.trainer.num_train_batches / self.plot_train_per_epoch
        else:
            x = self.trainer.epoch + 1
            n = self.trainer.num_val_batches / self.plot_valid_per_epoch
        self.board.draw(x, np.asarray(value), (
            'train_' if train else 'val_') + key, every_n=int(n))
	
    def training_step(self, batch):
        l = self.loss(self(*batch[:-1]), batch[-1])
        self.plot('loss', l, train=True)
        return l

    def validation_step(self, batch):
        l = self.loss(self(*batch[:-1]), batch[-1])
        self.plot('loss', l, train=False)

    def configure_optimizers(self):
        return tf.keras.optimizers.SGD(self.lr)

class DataModule(HyperParameters):
    def __init__(self, root = "../data"):
        self.save_hyperparameters()

	# Not implemented generically+
    def get_dataloader(self, train):
        raise NotImplementedError

    def train_dataloader(self):
        return self.get_dataloader(train = True)

    def val_dataloader(self):
        return self.get_dataloader(train = False)

    def get_tensorloader(self, tensors, train, indices=slice(0, None)):
        tensors = tuple(a[indices] for a in tensors)
        shuffle_buffer = tensors[0].shape[0] if train else 1
        return tf.data.Dataset.from_tensor_slices(tensors).shuffle(
            buffer_size=shuffle_buffer).batch(self.batch_size)

class Trainer(HyperParameters):
    def __init__(self, max_epochs, num_gpus=0, gradient_clip_val=0):
        self.save_hyperparameters()
        assert num_gpus == 0, 'No GPU support yet'

    def prepare_data(self, data):
        self.train_dataloader = data.train_dataloader()
        self.val_dataloader = data.val_dataloader()
        self.num_train_batches = len(self.train_dataloader)
        self.num_val_batches = (len(self.val_dataloader)
                                if self.val_dataloader is not None else 0)

    def prepare_model(self, model):
        model.trainer = self
        model.board.xlim = [0, self.max_epochs]
        self.model = model

    def fit(self, model, data):
        self.prepare_data(data)
        self.prepare_model(model)
        self.optim = model.configure_optimizers()
        self.epoch = 0
        self.train_batch_idx = 0
        self.val_batch_idx = 0
        for self.epoch in range(self.max_epochs):
            self.fit_epoch()

    def fit_epoch(self):
        raise NotImplementedError

    def prepare_batch(self, batch):
        return batch

    def fit_epoch(self):
        self.model.training = True
        for batch in self.train_dataloader:
            with tf.GradientTape() as tape:
                loss = self.model.training_step(self.prepare_batch(batch))
            grads = tape.gradient(loss, self.model.trainable_variables)
            if self.gradient_clip_val > 0:
                grads = self.clip_gradients(self.gradient_clip_val, grads)
            self.optim.apply_gradients(zip(grads, self.model.trainable_variables))
            self.train_batch_idx += 1
        if self.val_dataloader is None:
            return
        self.model.training = False
        for batch in self.val_dataloader:
            self.model.validation_step(self.prepare_batch(batch))
            self.val_batch_idx += 1

    def clip_gradients(self, grad_clip_val, grads):
        grad_clip_val = tf.constant(grad_clip_val, dtype=tf.float32)
        new_grads = [tf.convert_to_tensor(grad) if isinstance(
            grad, tf.IndexedSlices) else grad for grad in grads]
        norm = tf.math.sqrt(sum((tf.reduce_sum(grad ** 2)) for grad in new_grads))
        if tf.greater(norm, grad_clip_val):
            for i, grad in enumerate(new_grads):
                new_grads[i] = grad * grad_clip_val / norm
            return new_grads
        return grads

###
# Plotting
###

def use_svg_display():
	# Makes the notebook display matplotlib stuff as .svg files
	backend_inline.set_matplotlib_formats('svg')

def set_figsize(figsize = (3.5, 2.5)):
	use_svg_display
	plt.rcParams['figure.figsize'] = figsize

class ProgressBoard(HyperParameters):
	def __init__(self, xlabel = None, ylabel = None, xlim = None,
				ylim = None, xscale = 'linear', yscale = 'linear',
				ls = ['-', '--', '-.', ':'], colors = ['C0', 'C1', 'C2', 'C3'],
				fig = None, axes = None, figsize = (3.5, 2.5), display = True):
		self.save_hyperparameters()

	def draw(self, x, y, label, every_n=1):
		Point = collections.namedtuple('Point', ['x', 'y'])

		if not hasattr(self, 'raw_points'):
			self.raw_points = collections.OrderedDict()
			self.data = collections.OrderedDict()
		
		if label not in self.raw_points:
			self.raw_points[label] = []
			self.data[label] = []

		points = self.raw_points[label]
		line = self.data[label]
		points.append(Point(x, y))

		# Skips all the drawing steps which are not `every_n` multiples
		if len(points) != every_n:
			return
		
		# Lambda function which can be called by mean(x)
		mean = lambda x : sum(x) / len(x)

		line.append(Point(mean([p.x for p in points]),
						mean([p.y for p in points])))
		
		points.clear()

		# If the HyperParameter display is not true then we don't plot anything at all
		if not self.display:
			return
		
		use_svg_display()
		# If there is no matplotlib figure yet this instantiates it, otherwise we append
		if self.fig is None:
			self.fig = plt.figure(figsize=self.figsize)

		plt_lines, labels = [], []

		# This is probably a bit too convoluted, should break it up more
		# This glues together all plots, standard is 4 elements, if you want more you have to
		# adjust the corresponding hyperparameters in the initialization
		# Not sure, but I think zip is not strict zip([1, 2], ['a', 'b', 'c']) gives [(1, 'a'), (2, 'b')]
		for (k, v), ls, color in zip(self.data.items(), self.ls, self.colors):
			plt_lines.append(plt.plot([p.x for p in v], [p.y for p in v],
											linestyle=ls, color=color)[0])
		
			labels.append(k)

		# If self.axes exists then use that as axies otherwise it uses the current axes (gca = "Get Current Axes")
		axes = self.axes if self.axes else plt.gca()
		
		if self.xlim:
			axes.set_xlim(self.xlim)
		if self.ylim:
			axes.set_ylim(self.ylim)
		
		if not self.xlabel:
			self.xlabel = self.x
		
		axes.set_xlabel(self.xlabel)
		axes.set_ylabel(self.ylabel)
		
		axes.set_xscale(self.xscale)
		axes.set_yscale(self.yscale)
		
		axes.legend(plt_lines, labels)
		
		display.display(self.fig)
		display.clear_output(wait=True)