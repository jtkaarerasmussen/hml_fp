use std::{error::Error, vec};
use csv;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let in_path = args[1].to_string();
    let out_path = args[2].to_string();
    let do_circ:i32 = args[3].parse().unwrap();


    let ts;
    let ps;
    match read_csv(in_path){
        Ok(x) => {ts = x[0].clone(); ps = x[1].clone()},
        Err(e) => panic!("{:?}", e)
    }

    let mut k_out = vec![];
    if do_circ == 1 {
        for t in ts{
            k_out.push(k_circ(t,&ps))
        }
    } else {
        for t in ts{
            k_out.push(k(t,&ps))
        }
    }

    csv_write(k_out, out_path).unwrap();


}


fn csv_write(v:Vec<f64>, path:String) -> Result<(), Box<dyn Error>>{
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(vec_to_string(v))?;
    wtr.flush()?;
    Ok(())
} 

fn vec_to_string(v:Vec<f64>) -> Vec<String>{
    let mut out = vec![];
    for val in v{
        out.push(val.to_string());
    }
    out
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

fn k(t:f64, ps:&Vec<f64>) -> f64 {
    let mut total = 0;
    let mut i = 0;
    for p in ps.iter(){
        let mut j = 0;
        for q in ps.iter(){
            if (i != j) && ((p-q).abs() < t) {
                total += 1;
            }
            j+=1
        }
        i+=1;
    }
    (total as f64) / (ps.len() as f64).powi(2)
}

fn circ(t:f64) -> f64{
    if t < 0.5 {
        t
    } else {
        1.0-t
    }
}

fn k_circ(t:f64, ps:&Vec<f64>) -> f64 {
    let mut total = 0;
    let mut i = 0;
    for p in ps.iter(){
        let mut j = 0;
        for q in ps.iter(){
            if (i != j) && (circ((p-q).abs()) < t) {
                total += 1;
            }
            j+=1
        }
        i+=1;
    }
    (total as f64) / (ps.len() as f64).powi(2)
}