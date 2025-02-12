from bootstrap_sp_tool import *
import sys

taxis_vals = np.linspace(0,1,30)

f_ic = linspace_circ(1000,1000,5)
s_ic = linspace_circ(25,25,5)
m = 0 
d = 0.1

for t in taxis_vals :
    save_dir = f"/home/jonatank/Documents/HML/hml_fp/trial_runs/1dtaxis/1dtaxis_{sys.argv[1]}/d_0.1/t_{t}/"
    sp_run(t,m,d,f_ic,s_ic, save_dir)