# parse the sample logs and get all the
# reverse mapping instance ip addresses

ip_address = []
with open("sample_logs.log", "r") as log:
    lines = log.readlines()
    #print(lines)
    for line in lines:
        if "reverse mapping" in line:
            words = line.split("[")
            #print(words)
            ip_addr = words[2].split("]")
            #print(ip_addr[0])
            ip_address.append(ip_addr[0])
    log.close()

with open("ip_addr_out.txt", "w") as ip:
    for ip_addr in ip_address:
        ip.write(ip_addr.replace(".", "dot") + "\n")
    ip.close()