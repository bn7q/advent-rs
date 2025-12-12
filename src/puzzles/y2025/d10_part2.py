import os
import time
from ortools.sat.python import cp_model

script_dir = os.path.dirname(__file__)
rel_path = "../../../inputs/2025/d10.txt"
abs_file_path = os.path.join(script_dir, rel_path)
with open(abs_file_path, "r") as file:
    input = file.read()

start_time = time.time()

total_presses = 0

for line in input.splitlines():
    _, *buttons_str, joltages_str = line.split(' ')

    buttons = [ [ int(n) for n in btn[1:-1].split(",") ] for btn in buttons_str]
    target = [int(n) for n in joltages_str[1:-1].split(',')]
    max_joltage = max(target)

    solver = cp_model.CpSolver()
    model = cp_model.CpModel()

    button_vars = [model.new_int_var(0, max_joltage, str(b_i)) for b_i in range(len(buttons))]

    for j, target_j in enumerate(target):
        vars_j = [ button_vars[b_i] for b_i, b in enumerate(buttons) if j in b ]
        model.add( sum(vars_j) == target_j )

    model.minimize(sum(button_vars))

    solver.solve(model)
    presses = sum([solver.value(v) for v in button_vars])
    total_presses += presses

solution = total_presses
t = time.time() - start_time

print("=== Year 2025 - Day 10  ===")
print(f"Part 2: {solution}")
print(f"Took:   {t*1000:3f}ms")
