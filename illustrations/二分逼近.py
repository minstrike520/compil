low = 0
high = 2  # √2 一定在 0 到 2 之間
for i in range(100):
    mid = (low + high) / 2
    if mid * mid < 2:
        low = mid
    else:
        high = mid
print((low + high) / 2)
