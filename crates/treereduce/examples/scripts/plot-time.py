import seaborn as sns
import pandas as pd

cols = ["tool", "vers", "test", "jobs", "conf", "file", "start", "end", "time"]
data = pd.read_csv("data.csv", names=cols)
sns.set_theme()
plot = sns.catplot(data=data, kind="bar", x="tool", y="time")
plot.fig.savefig("out.svg") 
