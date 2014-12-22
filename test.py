from pymemcache.client import Client
c = Client(("localhost", 11211))

from time import time

c.flush_all(noreply=False)

start = time()
num = 1000
for x in range(num):
    c.set(str(x), str(x), noreply=False)
    val = c.get(str(x))
    assert val == str(x), str(x)

print "incrementing monkey (expecting a fail)"
result = c.incr("monkey", 1, noreply=False)
assert result == None

print "setting monkey"
print c.set("monkey", 0, noreply=False)
zero = c.get("monkey")
assert zero == "0", zero


print "incrementing monkey"
print c.incr("monkey", 1, noreply=False)

monkey = c.get("monkey")
assert monkey == "1", monkey

print c.decr("monkey", 1, noreply=False)

monkey = c.get("monkey")
assert monkey == "0", monkey

print num / (time() - start)



