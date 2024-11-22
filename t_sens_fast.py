import os
import subprocess
import numpy as np
import csv
import matplotlib.pyplot as plt
# import scipy
import matplotlib.patches as patches


def rust_k_circ(ts, vs):
    with open("temp.csv" , mode= "w") as f:
        wtr = csv.writer(f)

        wtr.writerow(ts)
        wtr.writerow(vs)

    rust_path = "~/Documents/HML/cleaning/ABM/fp/ripleys_k/target/release/ripleys_k"
    current_path = os.getcwd()
    
    os.system(f"{rust_path} {current_path}/temp.csv {current_path}/temp_out.csv 1")

    with open("temp_out.csv") as f:
        reader = csv.reader(f)

        for v in reader:
            out = [float(a) for a in v]
            break
    
    os.system(f"rm {current_path}/temp.csv")
    os.system(f"rm {current_path}/temp_out.csv")
    return out

def expectation_diff(vs, res=50):
    ts = np.linspace(0,0.5,res)
    ks = rust_k_circ(ts,vs)
    ex_ks = 2*ts
    diffs = np.abs(ks-ex_ks)
    return np.average(diffs), ts, ks

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

def run_multi(params, outputs, f_ic, s_ic):
    if len(params) != len(outputs):
        return "Failed"

    pros=[]
    for i in range(len(params)):
        with open(f"temp_ics{i}.csv", mode="w") as f:
            wtr = csv.writer(f)

            wtr.writerow(f_ic)
            wtr.writerow(s_ic)

        current_path = os.getcwd()
        csv_path = f"{current_path}/temp_ics{i}.csv"

        pro = subprocess.Popen(f"cd sp && cargo run {params[i][0]} {params[i][1]} {params[i][2]} {outputs[i]} {csv_path}", shell=True, stdout=subprocess.DEVNULL)
        pros.append(pro)
    
    for pro in pros:
        pro.wait()
    # os.system(f"rm {current_path}/temp_ics.csv")

def linspace_circ(clump_count, num):
    return np.array([np.linspace(1/(2*clump_count), 1 - (1/(2*clump_count)), clump_count)[i%clump_count] for i in range(num)])


ics = []
ics.append([np.random.rand(200), np.random.rand(25)])
ics.append([linspace_circ(200,200), linspace_circ(25,25)])
ics.append([linspace_circ(6,200),linspace_circ(10,25)])
ics.append([linspace_circ(3,200), linspace_circ(3,25)])
ics.append([linspace_circ(1,200), linspace_circ(1,25)])

# save_dir = "/media/jonatank/PG/HML/"
save_dir = "/home/jonatank/Documents/HML/cleaning/ABM/fp/taxis_quick_long/"

taxis = np.linspace(0,0.2,30)
diffs = 0.083

for ic_num in [0,1,2,3,4]:
    params = []
    outputs = []
    for t in taxis:
            params.append((t,0,diffs))
            outputs.append(f"{save_dir}t_ic_{ic_num}/taxis_{t}/")

    p_groups = []
    o_groups = []
    for i in range(len(params)//5):
        p_g = params[i*5: i*5+5]
        o_g = outputs[i*5: i*5+5]
        p_groups.append(p_g)
        o_groups.append(o_g)
    print(p_groups)
    # print(o_groups)

    for group_num in range(len(p_groups)):
        print(f"group: {group_num}/{30*30-1},   ic: {ic_num}")
        run_multi(p_groups[group_num],o_groups[group_num], ics[ic_num][0], ics[ic_num][1])