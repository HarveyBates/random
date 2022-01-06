import matplotlib.pyplot as plt

with open("data.txt", 'r') as f:
    rows = f.readlines()
    x, y = [], []
    for row in rows:
        xx, yy = row.split('\t')
        x.append(int(xx.strip()))
        y.append(int(yy.strip()))
    
    # Define a figure with 1x1 plot and a fig size of 6:6 x:y
    fig, ax = plt.subplots(1, 1, figsize=(6,6))

    # Define the line chart, add circle markers and a series label
    ax.plot(x, y, marker='o', label="Series 1")

    # Add a grid background
    ax.grid()
    
    # Set axis labels
    ax.set_xlabel("x")
    ax.set_ylabel("y")

    # Set axis limits
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 100)

    # Add a legend 
    ax.legend()

    # Add a title
    ax.set_title("Same plot")

    # Show the plot
    plt.show()
