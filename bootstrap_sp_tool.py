import numpy as np
import csv
import os 
import subprocess

def linspace_circ(clump_count, num, h_width=1):
    return np.array([np.linspace(h_width/(2*clump_count), h_width - (h_width/(2*clump_count)), clump_count)[i%clump_count] for i in range(num)])

def create_temp_csv( f_ic, s_ic, temp_path = "temp_ics.csv"):
    with open(temp_path, mode="w") as f:
        wtr = csv.writer(f)
        for row in [f_ic,s_ic]:
            wtr.writerow(row)

    current_path = os.getcwd()
    return current_path + f"/{temp_path}"


def sp_run(t,m,d, f_ic, s_ic, out_path):
    csv_path = create_temp_csv(f_ic, s_ic)

    pros = subprocess.Popen(f"cd sp/target/release && ./fp {t} {m} {d} {out_path} {csv_path}", shell=True, stdout=subprocess.DEVNULL)
    pros.wait()
    os.system(f"rm {csv_path}")


if __name__ == "__main__":
    f_ic = linspace_circ(1000,1000,5)
    s_ic = linspace_circ(25,25,5)
    t = 0
    m = 0.3
    d = 0.1
    save_dir = "/home/jonatank/Documents/HML/hml_fp/trial_runs/high_width_test/m/"
    # create_temp_csv(f_ic,s_ic)
    sp_run(t,m,d,f_ic,s_ic, save_dir)
