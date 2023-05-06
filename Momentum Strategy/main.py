import numpy as np
import pandas as pd

import math

def CalculateMomentum(df_dict, weights = [0.1, 0.2, 0.3, 0.4], MomentumTreshhold = 0, capital = 1000000):
    df_deltas = CalculateDelta(df_dict, weights = weights, Sorted = True)
    df_shares = CalculateShares(df_deltas, MomentumTreshhold = MomentumTreshhold, capital = capital)
    df = pd.concat([df_deltas[:len(df_shares)], df_shares], axis = 1)
    return df.set_axis(["One Month", "Three Months", "Six Months", "One Year", "Momentum (μ)", "Price", "Shares", "Share Value"], axis = 1)

def CalculateDelta(df_dict, weights = [0.1, 0.2, 0.3, 0.4], Sorted = True):
    '''
    Input:
        df_dict =   Dictionary of Dataframes which have the opening values of stocks, indexed by the stock ticker
        weights =   The weights for the strategy, 0 corresponds to 1 month difference, 1 to 3 month difference,
                        2 to 6 months difference and 3 to 1 year difference, have to sum to 1 for the strategy to make sense
        Sorted =    Whether the output should be sorted by Momentum
    Output:     Dataframe consisting of difference values, the momentum, and the current price of the corresponding stock
    '''
    deltas = {}
    for tick in df_dict.keys():
        loc = df_dict[tick].iloc
        oneMonth = df_dict[tick]["Open"][1] - df_dict[tick]["Open"][0]
        threeMonth = df_dict[tick]["Open"][3] - df_dict[tick]["Open"][0]
        sixMonth = df_dict[tick]["Open"][6] - df_dict[tick]["Open"][0]
        oneYear = df_dict[tick]["Open"][11] - df_dict[tick]["Open"][0]
        momentum = weights[0] * oneMonth + weights[1] * threeMonth + weights[2] * sixMonth + weights[3] * oneYear
        deltas[tick] = [oneMonth, threeMonth, sixMonth, oneYear, momentum, df_dict[tick]["Open"][0]]
        
    deltas_df = pd.DataFrame.from_dict(deltas, orient = "index", columns=["One Month", "Three Months", "Six Months", "One Year", "Momentum", "Price"])

    if(Sorted):
        deltas_df = deltas_df.sort_values(by="Momentum", ascending=False)

    return deltas_df

def CalculateShares(df_deltas, MomentumTreshhold = 0, capital = 1000000):
    df = df_deltas[df_deltas["Momentum"] > MomentumTreshhold]
    momentum_sum = df["Momentum"].sum()

    shares = {}
    shareValue = {}
    for row in df.iterrows():
        momentum_percentage = row[1]["Momentum"] / momentum_sum
        capital_over_price = capital / row[1]["Price"]
        shareNumber = math.floor(momentum_percentage * capital_over_price)
        shares[row[0]] = shareNumber
        shareValue[row[0]] = shareNumber * row[1]["Price"]
        
    return pd.DataFrame([shares, shareValue]).transpose()
