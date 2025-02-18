from bs_mp_f import *
import sys
import os
from concurrent.futures import ThreadPoolExecutor

def run(ic,t,m,d):
    f_ic = linspace_circ(1000,1000,5)
    s_ic = linspace_circ(25,25,5)
    fp = [[t,m,d],[t,m,d]]
    fcs = [1000,0]
    save_dir = f"{os.getcwd()}/trial_runs/taxis2d_full/i_{ic}/d_{d}/t_{t}/"
    mp_run(fp,fcs,f_ic,s_ic, save_dir,only_final=True)


t_vals = np.linspace(0,0.3,30)
d_vals = np.linspace(0,0.3,30)
ic_list = np.arange(10)

sim_inputs = []
for ic_num in ic_list:
    for d in d_vals:
        for t in t_vals:
            sim_inputs.append((ic_num,t,0,d))

with ThreadPoolExecutor(max_workers=10) as executor:
    executor.map(lambda a : run(*a), sim_inputs)
