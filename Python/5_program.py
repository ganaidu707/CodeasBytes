def fn(list=None):
    if list == None:
        list = []
    list.append('a')
    return list

print(fn())
print(fn())
print(fn())