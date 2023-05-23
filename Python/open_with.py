# difference between opening a file
# direct open and with open

f = open("temp.txt", "r")
print("direct open file content: ", f.read())
f.close()

with open("temp.txt", "r") as f:
    print("with open file content: ", f.read())
    
print("something")