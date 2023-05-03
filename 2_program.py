# what is the difference between "is" and "==" operator
# its a big mistake often do in python

a = [1, 2]
b = [1, 2]

def test_equal_op(x, y):
    return x == y
def test_is_op(x, y):
    return x is y

#print(test_equal_op(a, b))
#print(test_is_op(a, b))

# the "is" operation will return true if both side
# referencing same object where as "==" operator just
# checks both values same or not

# another case
string = input("enter a string: ")
# fix is need use "=="
if string == "code":
    print("correct")
else:
    print("incorrect")
    

    

