import numrust

primecount, eval_count = numrust.primecounter(
    range_from=0, range_til=500)
# returns 95 22279
print(primecount, eval_count)