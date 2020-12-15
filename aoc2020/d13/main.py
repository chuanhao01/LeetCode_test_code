depart_time = 1000510
bus_ids = [19, 41, 523, 17, 13, 29, 853, 37, 23]
min_diff = depart_time # Some arb large num
min_id = None # Temp
for bus_id in bus_ids:
    diff = (((depart_time // bus_id) + 1) * bus_id) - depart_time
    if diff < min_diff:
        min_diff = diff
        min_id = bus_id
print(min_diff, min_id)
print(min_diff * min_id)
        