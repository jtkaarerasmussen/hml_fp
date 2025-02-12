use rand;
use csv::WriterBuilder;
// use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;
use std::collections::VecDeque;
use std::fs::{self};
use std::error::Error;
use std::env;

fn main() {
    let max_iterations = 5000;

    // set constant parameters
    let station_count = 25;
    let fish_count = 200;
    let initial_energy = 3.0;
    let energy_loss_rate = 0.2;
    let birth_rate = 1;
    let fish_val:f64 = 0.1;

    // pull parameter values from cli
    let args:Vec<String> = env::args().collect();
    let out_path = args[1].to_string();

    // set stopping related parameters
    // let stop_req = 400;
    let stop_req = 1000;
    let stop_tol: f64 = 0.15;
    let equil_location = ((fish_count as f64)*fish_val + (birth_rate as f64)*initial_energy) / energy_loss_rate;

    // Generate vectors of Fish and Station instances based on ic file from cli 
    // (if not provided, uniform Fish, random Stations)
    let mut f = vec![];
    let mut s = vec![];
    if args.len() == 3{
        let ic_path = args[2].to_string();
        let ics = read_csv(ic_path).unwrap();
        for f_index in 0..ics[0].len(){
            f.push(Fish{
                taxis_rate:ics[1][f_index],
                mem_rate:ics[2][f_index],
                diff_rate:ics[3][f_index],
                loc:ics[0][f_index],
                mem:Memories::new()
            })
        };
        for loc in ics[4].clone(){
            s.push(Station{loc:loc, energy:initial_energy})
        };
    } else {
        for i in 0..fish_count{
            let f_init_loc = (i as f64) / (fish_count as f64);
            f.push(Fish{taxis_rate: 0.0, mem_rate: 0.0, diff_rate: 0.0, loc: f_init_loc, mem: Memories::new()});
        } 
        for _i in 0..station_count{
            let s_init_val = rand::random();
            s.push(Station{energy:initial_energy, loc:s_init_val});
        } 
    }

    // Format output dir
    match fs::remove_dir_all(out_path.clone()){
        Ok(_) => println!("remove dir worked"),
        Err(_e) => println!("remove dir did not work")
    };
    fs::create_dir_all(out_path.clone()).expect("make dir broke");

    // init sim with above ics
    let mut sim = Sim::new(station_count, fish_count, initial_energy);
    sim.fish = f;
    sim.stations = s;

    // init stopping trackers
    let mut equil_count = 0;
    let equil_lower = equil_location * (1.0 - stop_tol);
    let equil_upper = equil_location * (1.0 + stop_tol);
    let mut last_iter = max_iterations;

    // main sim loop
    for i in 0..max_iterations{
        // println!("{}", i);
        // save simulation state
        sim.csv(out_path.clone() + "/" + &i.to_string() + ".csv").expect("csv creation broke");

        // simulation actions 
        sim.sub_energy(energy_loss_rate); 
        sim.add_energy(fish_val, 5.0);
        sim.birth(birth_rate);
        sim.death();
        sim.taxis();
        sim.diffusion();
        sim.remember();

        // check if early stopping reqs are met
        if (sim.stations.len() as f64) < equil_upper && (sim.stations.len()as f64) > equil_lower {
            equil_count += 1;
        } else {
            equil_count = 0;
        }
        if equil_count >= stop_req {
            println!("Finished Early");
            last_iter = i+1;
            break;
        }

    }
        // output final state
        sim.csv(out_path.clone() + "/" + &last_iter.to_string() + ".csv").expect("csv creation broke");
}

struct Station{
    energy: f64,
    loc: f64
}

impl Station {
    fn new(initial_energy:f64) -> Station{
        Station { energy: initial_energy, loc: rand::random() }
    }
}

struct Memories {
    locations: VecDeque<f64>,
    qual: VecDeque<f64>
}

impl Memories {
    fn new() -> Memories {
        Memories{locations:VecDeque::from([0.0,0.0,0.0]), qual:VecDeque::from([0.0,0.0,0.0])}
    }

    fn add(&mut self, loc:f64, q:f64)  {
        self.locations.push_front(loc);
        self.qual.push_front(q);
    }
    fn rm_back(&mut self){
        self.locations.pop_back();
        self.qual.pop_back();
    }
    fn get_dists(&mut self, loc:f64) -> Vec<f64>{
        let mut out = vec![];
        for i in 0..3 {
           out.push(distance(loc, *self.locations.get(i).unwrap()));
        }
        out
    }
}

struct Fish {
    taxis_rate:f64,
    mem_rate: f64,
    diff_rate: f64,
    loc: f64,
    mem: Memories
}

impl Fish {
    fn new(t:f64,m:f64,d:f64) -> Fish{
        let q = rand::random();
        Fish{taxis_rate:t, mem_rate:m, diff_rate:d, loc:q, mem:Memories::new()}
    }
    
    fn get_mem_direction(&mut self, max_mem:f64, local_qual:f64) -> f64 {
        let dists = self.mem.get_dists(self.loc);
        let mut adjusted_quals = vec![local_qual];
        for i in 0..3 {
            adjusted_quals.push((0.75-0.1*(i as f64)) * (if max_mem<dists[i].abs() {0.0} else {1.0}) *self.mem.qual.get(i).unwrap())
        };
        let mut max_index = 0;
        let mut max = 0.0;
        for i in 0..=3{
            if adjusted_quals[i] > max{
                max_index = i;
                max = adjusted_quals[i];
            }
        }
        if max_index == 0 {
            return 0.0;
        } else {
            return dists[max_index - 1].clamp(-max_mem, max_mem);
        }
    }
}

// fn kernel(x:f64, loc:f64) -> f64{
//     0.5*(-(25.0*(x - loc)).powi(2)).exp()
// }

// wrapped
fn kernel(x:f64, loc:f64) -> f64 {
    let mut out = 0.0;
    for i in -1..=1{
        out += 0.5*(-(25.0*((x+(i as f64)) - loc)).powi(2)).exp();
    }
    out
}

fn distance(x1:f64, x2:f64) -> f64{
    let diff = x2 - x1;
    if diff < 0.5 {
        diff
    } else {
        1.0 - diff
    }
}

struct Sim {
    stations: Vec<Station>,
    fish: Vec<Fish>,
    initial_energy: f64
}

impl Sim {
    // fn new(station_count:u32, initial_energy:f64, f_ts:Vec<f64>, f_ms:Vec<f64>, f_ds:Vec<f64>) -> Sim{
    //     let mut v: Vec<Station> = vec![];
    //     for _ in 0..station_count{
    //         v.push(Station::new(initial_energy))
    //     } 
    //     let mut f: Vec<Fish> = vec![];
    //     let fish_count = f_ds.len();
    //     for i in 0..fish_count{
    //         f.push(Fish::new(f_ts[i], f_ms[i], f_ds[i]))
    //     } 
    //     Sim { stations: v, fish: f, initial_energy: initial_energy } 
    // }
    fn new(station_count:u32, fish_count:u32, initial_energy:f64) -> Sim{
        let mut v: Vec<Station> = vec![];
        for _ in 0..station_count{
            v.push(Station::new(initial_energy))
        } 
        let mut f: Vec<Fish> = vec![];
        for _ in 0..fish_count{
            f.push(Fish::new(0.0, 0.0, 0.0))
        } 
        Sim { stations: v, fish: f, initial_energy: initial_energy } 
    }

    fn birth(&mut self, birth_rate:u32){
        for _ in 0..birth_rate{
            self.stations.push(Station::new(self.initial_energy));
        }
    }

    fn death(&mut self){
        let mut marked = vec![];
        let mut i = 0;
        for s in self.stations.iter(){
            if s.energy <= 0.0 {
                marked.push(i);
            }
            i+=1;
        }
        for i in marked.iter().rev(){
            self.stations.remove(*i);
        }
    }

    fn sub_energy(&mut self, energy_loss_rate:f64){
        for s in self.stations.iter_mut(){
            s.energy -= energy_loss_rate
        }
    }

    fn get_sums(&self) -> Vec<f64>{
        let mut out = vec![];
        for f in self.fish.iter(){
            let mut total = 0.0;
            for s in self.stations.iter(){
                total += kernel(f.loc, s.loc)
            }
            out.push(total)

        }
        out
    }
    fn add_energy(&mut self, fish_val:f64, max_energy:f64){
        let sums = self.get_sums();
        let mut i=0;
        for f in self.fish.iter(){
            for s in self.stations.iter_mut(){
                let k = kernel(f.loc, s.loc);
                let min = if 1.0 > sums[i] {sums[i]} else {1.0};
                let new_energy = fish_val * (k / sums[i]) * min;
                s.energy += new_energy;
                if s.energy > max_energy{s.energy = max_energy;}
            }
            i+=1;
        }
    }

    fn csv(&self, path:String) -> Result<(), Box<dyn Error>>{
        let mut f_locs = vec![];
        let mut s_locs = vec![];
        for s in self.stations.iter(){
            s_locs.push(s.loc.to_string())
        }
        for f in self.fish.iter(){
            f_locs.push(f.loc.to_string())
        }

        let mut wtr = WriterBuilder::new().flexible(true).from_path(path)?;
        wtr.write_record(s_locs)?;
        wtr.write_record(f_locs)?;
        wtr.flush()?;
        Ok(())
    }

    fn taxis(&mut self){
        let kernal_res = 50;
        let mut k = vec![];
        for i in 0..kernal_res{
            let mut k_val = 0.0;
            for s in self.stations.iter(){
                k_val += kernel((i as f64)/((kernal_res-1) as f64), s.loc);
            }
            k.push(k_val)
        }

        for f in self.fish.iter_mut(){
            let max_steps = (f.taxis_rate*(kernal_res as f64)).round() as usize;
            if f.taxis_rate != 0.0{
                let start_index = (f.loc * ((kernal_res-1) as f64)).round() as usize;
                let right_index = if start_index == kernal_res-1 {0} else {start_index+1};
                let k0 = k[start_index];
                let k1 = k[right_index];
                let local_grad:i32 = if k1 > k0 {1} else {-1};

                let mut highest_k = 0.0;
                for step_offset in 0..max_steps{
                    let current_index = wrap_index((start_index as i32) + local_grad*(step_offset as i32), kernal_res);
                    if k[current_index] > highest_k{
                        highest_k = k[current_index];
                    } else {
                        f.loc = (wrap_index((current_index as i32)-local_grad , kernal_res)as f64) / (kernal_res as f64);
                        break; 
                    }
                }
                f.loc = wrap_val(f.loc);
            }
            if f.mem_rate != 0.0{
                let mut k0 = 0.0;
                for s in self.stations.iter(){
                    k0 += kernel(f.loc,s.loc)
                }
                let mem_dir = f.get_mem_direction(f.mem_rate, k0);
                f.loc += mem_dir;
            }

            f.loc = wrap_val(f.loc)
        }
        // println!("{}",mem_count);
    }

    fn diffusion(&mut self){
        for f in self.fish.iter_mut(){
            let mut move_size:f64 = rand::random();
            move_size = f.diff_rate * 2.0 * (move_size - 0.5);
            f.loc += move_size;
            // f.loc = f.loc.clamp(0.0, 1.0);
            f.loc = wrap_val(f.loc);
        }
    }

    fn remember(&mut self) {
        for f in self.fish.iter_mut() {
            let mut qual = 0.0;
            for s in self.stations.iter(){
                qual += kernel(f.loc,s.loc)
            }
            f.mem.add(f.loc, qual);
            f.mem.rm_back();
        }
    }
}

fn read_csv(path:String) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let mut rdr  = csv::ReaderBuilder::new().flexible(true).has_headers(false).from_path(path)?;
    let mut out = vec![];
    for result in rdr.records() {
        let record = result?;
        let mut out1 = vec![];
        for r in record.iter(){
            out1.push(r.parse().unwrap());
        }
        out.push(out1);
    }
    Ok(out)
}

fn wrap_val(v:f64) -> f64{ 
    if v < 0.0{
        1.0 + v
    } else if v > 1.0{
        v-1.0
    }else {
        v
    }
}

fn wrap_index(i:i32, kernel_res:usize) -> usize{
    if i <= -1 { 
        ((kernel_res as i32)+i) as usize
    }else if i >= kernel_res as i32 { 
        (i - (kernel_res as i32)) as usize
    } else {
        i as usize
    }
}