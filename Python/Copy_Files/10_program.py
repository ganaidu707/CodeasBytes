# copying files from one location to
# another location
import os
import shutil

source = "src\\"
destination = "dest\\"

if os.path.exists(source) and os.path.exists(destination):
   files = os.listdir(source)
   
   for file_name in files:
      shutil.copy(source + file_name, destination + file_name)
      print("file: {} copied".format(file_name))
      
   print("all files are copied")