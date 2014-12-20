from pymemcache.client import Client
c = Client(("localhost", 11211))

c.set("test", "val", noreply=False)
val = c.get('test')

assert val == "val"

