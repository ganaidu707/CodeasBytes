inp = input("enter a social media name: ")
output = "hello " + inp
print(output)
with open('output.txt', 'w') as f:
    f.write(output)
    f.close()