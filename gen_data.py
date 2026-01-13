import struct
import random
import json

ROWS = 1_000_000

# 1. Generate Binary Data
with open("id.bin", "wb") as f_id, \
     open("age.bin", "wb") as f_age, \
     open("salary.bin", "wb") as f_sal, \
     open("dept.bin", "wb") as f_dep:

    for i in range(ROWS):
        f_id.write(struct.pack("i", i))                  # ID: 0 to 999,999
        f_age.write(struct.pack("i", random.randint(18, 65))) # Age: 18-65
        f_sal.write(struct.pack("d", random.uniform(30000, 150000))) # Salary: Float64
        f_dep.write(struct.pack("i", random.randint(1, 5))) # Dept: 1-5

# 2. Match the Metadata to the data
metadata = {
  "name": "corporate_data",
  "columns": [
    { "name": "id", "data_type": "Int32", "file": "id.bin" },
    { "name": "age", "data_type": "Int32", "file": "age.bin" },
    { "name": "salary", "data_type": "Float64", "file": "salary.bin" },
    { "name": "dept", "data_type": "Int32", "file": "dept.bin" }
  ],
  "departments": [
    {"id": 1, "name": "Programming"},
    {"id": 2, "name": "Finance"},
    {"id": 3, "name": "Cybersecurity"},
    {"id": 4, "name": "Data Science"},
    {"id": 5, "name": "DevOps"}
  ]
}

with open("metadata.json", "w") as f:
    json.dump(metadata, f, indent=2)

print(f"Done! Created {ROWS} rows across 4 files and updated metadata.json.")