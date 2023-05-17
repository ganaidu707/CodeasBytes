list1 = ["name", "age", "skills"]
list2 = ["chris", 30, "python"]

my_dict1 = dict(zip(list1, list2))
print(my_dict1)

my_dict2 = {"colour": "red"}
print(my_dict2)

#my_dict1.update(my_dict2)

my_dict = {}
#my_dict.update(my_dict1)
#my_dict.update(my_dict2) #python 3.4
#my_dict = {**my_dict1, **my_dict2} # python 3.5 to 3.9

my_dict = my_dict1 | my_dict2
print(my_dict)

