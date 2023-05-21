# set data type in python
# set = {}

list = ["hi", "hello", "hola", "oi", "hi"]
x1 = set(list)
print(x1)

tuple = ("what", "why", "who", "how", "oi")
x2 = set(tuple)
print(x2)

#operations on sets
#x = x1 | x2 #union
#print(x)

#x = x1 & x2 #intersection
#print(x)

#x = x1 - x2 #difference
#print(x)

x = x1 ^ x2 #symmetric difference
print(x)