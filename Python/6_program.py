def func1():
    list1 = [1, 2, 3]
    list2 = list1
    print(id(list1), id(list2))
    list1 += [4, 5, 6]
    print(id(list1), id(list2))

def func2():
    list1 = [1, 2, 3]
    list2 = list1
    print(id(list1), id(list2))
    list1 = list1 + [4, 5, 6]
    print(id(list1), id(list2))
    
print("calling function 1")
func1()
print("calling fucntion 2")
func2()