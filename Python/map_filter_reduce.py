# map, filter, reduce

from functools import reduce

numbers = [1, 2, 3, 5, 6]
'''def cube(x):
    return x**3'''

cube_nums = list(map(lambda x: x**3, numbers))
print(cube_nums)

'''def is_even(x):
    return x % 2 == 0'''

even_nums = list(filter(lambda x: x%2 == 0, numbers))
print(even_nums)

'''def add(x, y):
    return x + y'''
add_nums = reduce(lambda x, y: x+y, numbers)
print(add_nums)