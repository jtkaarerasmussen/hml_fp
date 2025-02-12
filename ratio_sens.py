from bootstrap_mp_tool import *

ic_reps = 10
ratio_res = 100
save_dir_prefix = "/home/jonatank/Documents/HML/hml_fp/trial_runs/ratio_test_1/"
f_ic = linspace_circ(200,200)
s_ic = linspace_circ(25,25)

f_params = [[0.05,0,0.1],[0.25,0,0.1]]

for ratio in np.linspace(0,1,ratio_res):
    fcs = [int(200*ratio), 200-int(200*ratio)]
    print(fcs)
    for ic_rep in range(ic_reps):
        save_dir = f"{save_dir_prefix}/r_{ratio}/{ic_rep}"
        mp_run(f_params,fcs,f_ic,s_ic, save_dir)

