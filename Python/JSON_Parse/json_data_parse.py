# rest api json data parsing
import requests #pythom -m pip install requests

url = "https://openlibrary.org/api/books?\
bibkeys=ISBN:0201558025,LCCN:93005405&format=json"

response = requests.get(url)
#print(response.status_code)

if response.status_code == 200:
    output = response.json()
    #print(output)
    for item in output:
        print(output[item]["preview"])
else:
    print("get response failed reason {}".format(response.text))