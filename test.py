import pickle
from demo import square, sub1

print(square(5))
print(square(10))
print(sub1.cube(10))

# error
pickle.dumps(sub1.cube)

# also error
cube = sub1.cube
pickle.dumps(cube)

# cannot import without using sys.modules trick
from demo.sub1 import cube