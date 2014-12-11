import socket

s = socket.socket(socket.AF_INET)
s.connect(("localhost", 11211))

s.send("SET jon\nhaddad\r\n")

s.send("GET jon\r\n")

print "DONE"

