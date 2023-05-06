#https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=IBM&interval=5min&outputsize=full&apikey=demo
def APICallIntraday(symbol, interval, outputsize = "compact"):
    api_base = "https://www.alphavantage.co"
    api_function = "function=TIME_SERIES_INTRADAY"
    api_symbol = f"symbol={symbol}"
    api_interval = f"interval={interval}"
    api_outputsize = f"outputsize={outputsize}" # Either "compact" or "full"
    api_datatype = "datatype=json"
    return f"{api_base}/query?{api_function}&{api_symbol}&{api_interval}&{api_outputsize}&{api_datatype}&apikey={API_KEY}"

interval = "5min"
req = requests.get(APICallIntraday("AAPL", interval, outputsize = "full"))
data = req.json()
df = pd.DataFrame.from_dict(data[f"Time Series ({interval})"], orient = "index").sort_index()

plt.gca().xaxis.set_major_formatter(mdates.DateFormatter("%H:%M"))
plt.gca().xaxis.set_major_locator(mdates.HourLocator(interval=2))
plt.plot(TimeList_oneDay, low_oneDay)
plt.plot(TimeList_oneDay, open_oneDay)
plt.plot(TimeList_oneDay, high_oneDay)
plt.gcf().autofmt_xdate()
plt.show()