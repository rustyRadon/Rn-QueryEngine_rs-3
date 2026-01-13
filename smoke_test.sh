#!/bin/bash

echo "--- STARTING ENGINE SMOKE TEST ---"
echo "Generating fresh 1,000,000 row dataset..."
python3 gen_data.py

echo -e "\n[TEST 1] Standard Query: Dept 1 (Programming)"
cargo run --release -- 1

echo -e "\n[TEST 2] High Bar: Dept 2, Age > 40, Salary > 90,000"
cargo run --release -- 2 40 90000

echo -e "\n[TEST 3] Mid-Level: Dept 4 (Data Science), Age > 25, Salary > 50,000"
cargo run --release -- 4 25 50000

echo -e "\n[TEST 4] The 'Ghost' Query: Non-existent Dept (99)"
cargo run --release -- 99

echo -e "\n[TEST 5] The 'Entry Level' Query: Dept 1, Age > 18, Salary > 20,000"
cargo run --release -- 1 18 20000

echo -e "\n--- SMOKE TEST COMPLETE ---"