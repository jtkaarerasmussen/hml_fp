import numpy as np
from bs_mp_f import *
import sys
import os
from concurrent.futures import ThreadPoolExecutor

def run(ic_num,p_set,c,f_ic,s_ic,t,d):
    f_ic = linspace_circ(1000,1000,5)
    s_ic = linspace_circ(25,25,5)
    fp = [[t,0,d],[t,0,d]]
    fcs = [1000,0]
    save_dir = f"{os.getcwd()}/trial_runs/ic_hist/ic_{ic_num}/p_{p_set}/{c}/"
    mp_run(fp,fcs,f_ic,s_ic, save_dir,only_final=True)


ic_list = np.arange(100)
param_sets = [(0.25,0.16),(0.05,0.25),(0.1,0.1)]
f_ic = [np.random.rand(1000)*5, linspace_circ(1000,1000,5), np.array([2.5 for _ in range(1000)])]
s_ic = [np.random.rand(25)*5, linspace_circ(25,25,5), np.array([2.5 for _ in range(25)])]

sim_inputs = []
for ic_num in range(3):
    for p_s in range(3):
        for c in range(100):
            sim_inputs.append((ic_num,p_s,c,f_ic[ic_num],s_ic[ic_num],param_sets[p_s][0],param_sets[p_s][1]))

with ThreadPoolExecutor(max_workers=10) as executor:
    executor.map(lambda a : run(*a), sim_inputs)
