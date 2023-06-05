global global_var

def global_var_func1():
    global global_var
    global_var = 10
    print("global var at func1 = ", global_var)

def global_var_func2():
    global_var = 20
    print("global var at func2 = ", global_var)

global_var_func1()
print("global var value = ", global_var)
global_var_func2()
print("global var value = ", global_var)