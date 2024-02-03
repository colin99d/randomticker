import pandas as pd

df = pd.read_csv("data/equities.csv")
raw_symbols = df.symbol.to_list()
final_symbols = []
for symbol in raw_symbols:
    if not isinstance(symbol, str):
        continue
    if "." in symbol:
        continue
    if "-" in symbol:
        continue
    if "^" in symbol:
        continue
    final_symbols.append(symbol.strip())

final = ""
for symbol in final_symbols:
    final += f'"{symbol}",\n'
with open("output.txt", "w") as text_file:
    text_file.write(final)
