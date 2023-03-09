import numrust

primecount, eval_count = numrust.primecounter(
    range_from=0, range_til=500)
# returns 95 22279
print(primecount, eval_count)
t1 = numrust.Tensor([1, 2, 3, 9], [2, 2])
t2 = numrust.Tensor([5, 6, 7, 8], [2, 2])

# add the two tensors
t3 = numrust.Tensor.div(t1, t2)
print(t3.data)
print(t3.shape)
