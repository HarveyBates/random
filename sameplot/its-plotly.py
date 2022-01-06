import plotly.express as px

with open("data.txt", 'r') as f:
    rows = f.readlines()
    x, y = [], []
    for row in rows:
        xx, yy = row.split('\t')
        x.append(int(xx.strip()))
        y.append(int(yy.strip()))
    
    fig = px.line(x=x, y=y, title="Same Plot")
    fig.show()
