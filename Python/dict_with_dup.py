# python dictionaries with duplicate key
# Python dictionaries don't support duplicate keys. 
# One way around is to store lists or sets inside the dict.
from collections import defaultdict

my_keys = [1, 2, 2, 3, 3, 3]
my_values = ['a', 'b', 'c', 'd', 'e', 'f']
i = 0

# method 1
# One easy way to achieve this is by using "defaultdict"
data_dict = defaultdict(list)
for k in my_keys:
    data_dict[k].append(my_values[i])
    i += 1
print(data_dict)

#2nd method
results = {}
i = 0
for k in my_keys:
    results.setdefault(k, []).append(my_values[i])
    i += 1
    
print(results)