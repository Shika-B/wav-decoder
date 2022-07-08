import matplotlib.pyplot as plt
import numpy as np
import ast

with open("plot_points.txt") as file:
    points = file.read()
    l = ast.literal_eval(points)
xs = list(range(len(l)))

plt.plot(np.array(xs[0:100]), np.array(l[0:100]))
plt.show()
