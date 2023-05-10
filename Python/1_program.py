# division(/) and floor divisions (//) in python

a = 5
print(a/2)
print(a//2)

a = 6
if a / 2 == a // 2:
    print("same")
else:
    print("not same")

# same becasue it will be 3 in both the cases

a = 1000000000000000000006
if a / 2 == a // 2:
    print("same")
else:
    print("not same")
    
# what would be the output?
# "same" or "not same"?
# as per the above logic for the even numbers it shall be same
print(a/2)
print(a//2)