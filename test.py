from pymemcache.client import Client
c = Client(("localhost", 11211))

from time import time

start = time()
num = 10000
for x in range(num):
    c.set("test", "val", noreply=False)

print num / (time() - start)

val = c.get('test')

assert val == "val", val

