from pymemcache.client import Client
c = Client(("localhost", 11211))

c.set("test", "val")
val = c.get('test')

assert val == "val"

