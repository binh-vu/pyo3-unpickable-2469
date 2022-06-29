import pickle
from demo import square, sub1
from demo.sub1 import cube

print(square(5))
print(square(10))
print(sub1.cube(10))

pickle.dumps(sub1.cube)

cube = sub1.cube
pickle.dumps(cube)