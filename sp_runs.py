from bootstrap_sp_tool import *
import sys
import os

taxis_vals = np.linspace(0,0.3,30)
mem_vals = np.linspace(0,0.3,30)

f_ic = linspace_circ(1000,1000,5)
s_ic = linspace_circ(25,25,5)

base_dir = os.getcwd()+"/"

for i in range(10):
    m = 0 
    d = 0.1

    for t in taxis_vals :
        save_dir = f"{base_dir}trial_runs/1dtaxis/1dtaxis_{i}/d_0.1_t_{t}/"
        sp_run(t,m,d,f_ic,s_ic, save_dir)


    t = 0 
    d = 0.1

    for m in mem_vals :
        save_dir = f"{base_dir}trial_runs/1dmem/1dmem_{i}/d_0.1_m_{m}/"
        sp_run(t,m,d,f_ic,s_ic, save_dir)