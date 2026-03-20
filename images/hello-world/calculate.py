import sys
import json

a = float(sys.argv[1])
b = float(sys.argv[2])
op = sys.argv[3]

if op == "add":
    result = a + b
elif op == "multiply":
    result = a * b
else:
    print(json.dumps({"error": f"unknown op: {op}"}))
    sys.exit(1)

print(json.dumps({"result": result}))
