from bootstrap_sp_tool import *
import sys
import os

taxis_vals = np.linspace(0,0.3,30)
d_vals = np.linspace(0,0.3,30)

f_ic = linspace_circ(1000,1000,5)
s_ic = linspace_circ(25,25,5)

base_dir = os.getcwd()+"/"
m=0

for i in range(5):
    for t in taxis_vals :
        for d in d_vals:
            save_dir = f"{base_dir}trial_runs/2dtaxis/{i}/d_{d}/t_{t}/"
            sp_run(t,m,d,f_ic,s_ic, save_dir)