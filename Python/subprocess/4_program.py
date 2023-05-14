# subprocess in python

import subprocess

p =  subprocess.run(["python", "test_run.py"],
        capture_output=True,
        text=True,
        input="instagram",
        check=True)

if p.returncode != 0:
        print("stdout: ", p.stdout)
        print("stderr: ", p.stderr)