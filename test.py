from pymemcache.client import Client
c = Client(("localhost", 11211))

from time import time

start = time()
num = 1000
for x in range(num):
    c.set("test", str(x), noreply=False)
    val = c.get('test')
    assert val == str(x), str(x)

print num / (time() - start)



