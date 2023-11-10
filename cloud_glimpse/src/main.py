import laspy as lp
import numpy as np

las = lp.read("points.las")
h = las.header
las.header.point_count

xyz = las.xyz

a = np.all(xyz[..., 0])

print(xyz)
