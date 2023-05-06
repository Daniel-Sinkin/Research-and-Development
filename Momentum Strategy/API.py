# Allows API access to Yahoo Finance
import yfinance as yf

import time

def APICallMultiprocessing(tickers, processCount = 6):
	size = len(tickers) // processCount

	tickers_split = []
	for i in range(processCount):
		tickers_split.append(tickers[(i * size):((i + 1) * size)])

	tail = tickers[(processCount * size):]
	if(tail != []):
		tickers_split.append(tail)
		processCount = processCount + 1

	queue = mp.Queue()

	processPool = []
	for i in range(processCount):
		processPool.append(mp.Process(target = API.APICallMultiprocessingThread, args=(queue, tickers_split[i],)))

	for p in processPool:
		p.start()

	df_dict = {}
	while(not queue.empty()):
		curr = queue.get()
		df_dict[curr[0]] = curr[1]

	return df_dict

def APICallMultiprocessingThread(queue, tickers):
	retVal = []

	for tick in tickers:
		print(tick)
		ticker = yf.Ticker(tick)

		hist = ticker.history(period="5y", interval = "1mo")
		queue.put([tick, hist])

def APICall(tickers, Output = False, Time = False):
	'''
	Input:
		Tickers = A list of strings, corresponding to a valid Ticker
		Output = If every API call should be printed
		Time = If the API call should be timed, prints the duration after the API call
	Output:		Dictionary of Dataframes consisting of the stocks 1 year history
	'''
	if(Time):
		time_start = time.time()

	n = len(tickers)

	i = 1
	df_dict = {}
	for tick in tickers:
		if(Output):
			print(f"<{str(i).zfill(3)}/{n}> Calling the API for {tick}")

		ticker = yf.Ticker(tick)

		df_dict[tick] = ticker.history(period="5y", interval = "1mo")

		i = i + 1

	if(Time):
		time_end = time.time()
		print(f"The API calls took a total of {time_end - time_start} seconds.")

	return df_dict
