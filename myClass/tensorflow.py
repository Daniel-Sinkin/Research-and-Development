import tensorflow as tf
import numpy as np
import inspect

# Package for plotting
from matplotlib import pyplot as plt

from matplotlib_inline import backend_inline

###
# Plotting
###

def use_svg_display():
	# Makes the notebook display matplotlib stuff as .svg files
	backend_inline.set_matplotlib_formats('svg')

def set_figsize(figsize = (3.5, 2.5)):
	use_svg_display
	plt.rcParams['figure.figsize'] = figsize

def plot_data(X, Y):
	plt.plot(X, Y, marker = '', linestyle = '-', markersize = 8)
	plt.xlabel('X-axis')
	plt.ylabel('Y-axis')
	plt.title('Plot of X and Y sets')

	plt.xlim(min(X)-1, max(X)+1)
	plt.ylim(min(Y)-1, max(Y)+1)

	plt.show()

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

