from pymemcache.client import Client
from time import time

lim = 50000
def run_test():
    c = Client(("localhost", 11211))
    start = time()
    for x in range(lim):
        c.set(str(uuid1()), "1")

    return time()-start


from multiprocessing import Pool, Process

total_procs = 10

procs = []

from uuid import uuid1

start = time()
for x in range(total_procs):
    tmp = Process(target=run_test)
    tmp.start()
    procs.append(tmp)

print "Waiting for procs to join.."

for x in procs:
    x.join()
    print "Done {}".format(x)

total_time = time()-start
rate = (lim * total_procs) / total_time
print "total time: {} {}/s".format(time() - start, rate)


