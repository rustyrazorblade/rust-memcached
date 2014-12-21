from pymemcache.client import Client
c = Client(("localhost", 11211))

from time import time

start = time()
num = 1000
for x in range(num):
    c.set(str(x), str(x), noreply=False)
    val = c.get(str(x))
    assert val == str(x), str(x)

print "setting monkey"
c.set("monkey", 0, noreply=False)

print "incrementing monkey"
c.incr("monkey", 1, noreply=False)


print num / (time() - start)



