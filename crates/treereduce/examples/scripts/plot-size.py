import seaborn as sns
import pandas as pd

cols = ["tool", "vers", "test", "jobs", "conf", "file", "start", "end size", "time"]
data = pd.read_csv("data.csv", names=cols)
sns.set_theme()
plot = sns.catplot(data=data, kind="bar", x="tool", y="end size")
plot.fig.savefig("out.svg") 
