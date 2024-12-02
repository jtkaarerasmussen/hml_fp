import os
import subprocess
import numpy as np
import csv
import matplotlib.pyplot as plt
# import scipy
import matplotlib.patches as patches



def mem_run(taxis, mem_rate, diff, output, f_ic, s_ic):
    with open("temp_ics.csv", mode="w") as f:
        wtr = csv.writer(f)

        wtr.writerow(f_ic)
        wtr.writerow(s_ic)

    current_path = os.getcwd()
    csv_path = current_path + f"/temp_ics.csv"

    pros = subprocess.Popen(f"cd sp && cargo run {taxis} {mem_rate} {diff} {output} {csv_path}", shell=True, stdout=subprocess.DEVNULL)
    pros.wait()
    os.system(f"rm {current_path}/temp_ics.csv")

def linspace_circ(clump_count, num):
    return np.array([np.linspace(1/(2*clump_count), 1 - (1/(2*clump_count)), clump_count)[i%clump_count] for i in range(num)])


ics = []
ics.append([np.random.rand(200), np.random.rand(25)])
ics.append([linspace_circ(200,200), linspace_circ(25,25)])
ics.append([linspace_circ(6,200),linspace_circ(10,25)])
ics.append([linspace_circ(3,200), linspace_circ(3,25)])
ics.append([linspace_circ(1,200), linspace_circ(1,25)])

save_dir = "/home/jonatank/Documents/HML/cleaning/ABM/fp/taxis_quick_long/"

taxis = np.linspace(0,0.5,30)
diffs = np.linspace(0,0.3,30)

for ic_num in range(len(ics)):
    for t in taxis:
        for d in diffs:
            mem_run(t,0,d, f"{save_dir}t_ic_{ic_num}/taxis_{t}/diffs_{d}/", ics[ic_num][0], ics[ic_num][1])

