import pandas as pd

frame = pd.read_excel("c:/Users/sak-a/Documents/test.xlsx")
score = frame.iloc[2:22, 5]
print(score)
sum = score.sum()
min = score.min()
max = score.max()
cnt = score.count()
print(sum)
print(min)
print(max)
print(f"soces is {round((sum - min - max) / cnt, 2)}")
