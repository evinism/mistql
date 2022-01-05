from mistql import query
import json

q = input("Enter a query: ")
d = input("Enter data (blank for none): ")
if d == "":
    d = {}
else:
    d = json.loads(d)
print(query(q, d))
