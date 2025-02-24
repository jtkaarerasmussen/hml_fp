import numpy as np
import csv
import os 
import subprocess

def linspace_circ(clump_count, num):
    return np.array([np.linspace(1/(2*clump_count), 1 - (1/(2*clump_count)), clump_count)[i%clump_count] for i in range(num)])

def create_temp_csv(fish_params, fish_counts, f_ic, s_ic, temp_path = "temp_ics.csv"):
    ic_csv_arr = [f_ic]
    params=[]
    for param_index, fish_count in enumerate(fish_counts):
        for _ in range(fish_count):
            params.append(fish_params[param_index])
    params = np.transpose(np.array(params))
    for p in params:
        ic_csv_arr.append(p)
    ic_csv_arr = np.transpose(ic_csv_arr)
    np.random.shuffle(ic_csv_arr)
    ic_csv_arr = np.transpose(ic_csv_arr)


    with open(temp_path, mode="w") as f:
        wtr = csv.writer(f)
        for row in ic_csv_arr:
            wtr.writerow(row)
        wtr.writerow(s_ic)

    current_path = os.getcwd()
    return current_path + f"/{temp_path}"

def mp_run(fish_params, fish_counts, f_ic, s_ic, out_path):
    csv_path = create_temp_csv(fish_params, fish_counts, f_ic, s_ic)

    pros = subprocess.Popen(f"cd mp/target/release && ./fp {out_path} {csv_path}", shell=True, stdout=subprocess.DEVNULL)
    pros.wait()
    os.system(f"rm {csv_path}")
    create_temp_csv(fish_params, fish_counts, f_ic, s_ic, out_path+"/ic.csv")

if __name__ == "__main__":
    f_ic = linspace_circ(200,200)
    s_ic = linspace_circ(25,25)
    fp = [[0.2,0,0],[0,0,0.2]]
    fcs = [100,100]
    save_dir = "/home/jonatank/Documents/HML/hml_fp/trial_runs/multi_bootstrap_test/"
    mp_run(fp,fcs,f_ic,s_ic, save_dir)
