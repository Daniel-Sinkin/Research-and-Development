import requests

bannedTickers = ["GEHC", "ICE", "GEN", "FOX", "CEG", "CARR", "CDAY", "CTVA", "DOW", "FOXA", "MRNA", "OGN", "OTIS"]

def SNP500Tickers(BannedTickers = bannedTickers, WriteToFile = False, Sorted = False):
    '''
    Input:
        BannedTickers   = Tickers which should be skipped
        WriteToFile     = Bool, Whether or not the Tickers are written to a file
        Sorted          = If the output should be sorted or the order of the Website gets preserved
    Output: List of Strings consisting of all of the SNP500 Tickers which
            don't contain "." and aren't contained in the BannedTickers input
    '''

    # This takes the Wikipedia site on the SnP500, there is only one column of hyperlinks,
    # so it extracts for every row in the table the only hyperlink, a more general but slighty
    # longe approach would be to extract the table, and the first column of every row
    req_SNPWIKI = requests.get("https://en.wikipedia.org/wiki/List_of_S%26P_500_companies")
    # split("tbody") seperates the table, "split("<tr>"")" seperates into the individual rows
    # tr = table-rows <tr> ... </tr> is one row
    rows = req_SNPWIKI.text.split("tbody")[1].split("<tr>")

    retVal = []
    for r in rows[2:]:
        # a = hyperlink, <a> ... </a> is one hyperlink, this is always the ticker
        curr = r.split("</a>")[0].split(">")[-1]

        # Sometimes Tickers have ".", the API doesn't like this
        if "." not in curr and curr not in BannedTickers:
            retVal.append(curr)

    if(Sorted):
        retVal = sorted(retVal)

    if(WriteToFile):
        with open("Data/SNP500.csv", "w") as f:
            for t in retVal:
                f.write(t + "\n")

    return retVal
