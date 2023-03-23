use std::{f64::consts::PI,f64,f64::consts::E};
use itertools_num::{linspace,Linspace};
use std::fs::File;
use csv;


/// save the evolved system to a csv file, iterate over initial conditions x0 and y0
/// 
/// f is the function used for evolution. For example use standard_map or var
pub fn save_map(filename: &str, x0_all: Linspace<f64>, y0_all: Linspace<f64>, f: fn(f64,f64,f64, usize) -> [Vec<f64>; 2], k: f64, t: usize, pairs: bool){
    let file = File::create(&filename).unwrap();
    let time = std::time::Instant::now();
    let mut wtr = csv::Writer::from_writer(&file);

    if pairs{
        for (x0, y0) in x0_all.clone().zip(y0_all.clone()){
            let [x,y] = f(x0, y0, k, t);
            
            for i in 0..t+1{
                wtr.write_record(&[x[i].to_string(), y[i].to_string()]).unwrap();
            }
        }
    } 
    else{
        for y0 in y0_all.clone(){
            for x0 in x0_all.clone(){
                let [x,y] = f(x0, y0, k, t);
                
                for i in 0..t+1{
                    wtr.write_record(&[x[i].to_string(), y[i].to_string()]).unwrap();
                }
            }  
        }
    }        
    
    println!("{} written, \n\t time elapsed: {} ms", filename, time.elapsed().as_millis() );
    println!("\t parameters: \n\t\t x0_all: {:?} \n\t\t y0_all: {:?} \n\t\t K: {}", x0_all, y0_all, k);
}


/// Calculate the Standard Chirikov evolution from the initial conditions x0, y0.
/// 
/// k controls the chaoticity of the system, larger values lead to more chaotic behaviour.
/// 
/// y\[i+1\] = y\[i\] - (k/(2.0*PI))*(2.0*PI*x\[i\]).sin();
/// 
/// x\[i+1\] = x\[i\] + y\[i+1\];
pub fn standard_map(x0: f64, y0: f64, k: f64, t: usize) -> [Vec<f64>; 2]{
   
    let mut x = vec![0.0; t+1];
    let mut y = vec![0.0; t+1];
    x[0] = x0;
    y[0] = y0;
    // evolve
    for i in 0..t{
        y[i+1] = y[i] - (k/(2.0*PI))*(2.0*PI*x[i]).sin();
        x[i+1] = x[i] + y[i+1];
        x[i+1] -= x[i+1].floor() //turn off for unstable and stable manifolds
    }
    // let x = x.iter().map(|x| x + 0.0).collect::<Vec<_>>();
    
    [x, y]
}


/// normalized eigenvector from x
pub fn eigenvector(x: f64, k: f64, stable: bool) -> Vec<f64>{
    let cos: f64 = (2.*PI*x).cos();
    let kp: f64 = k * cos;
 
    // eigenvalues
    let la: f64 =  (2. - kp)/2.0;
    let lb: f64 =  (((kp.powi(2)) - 4.*kp).sqrt())/2.0;
    let l1: f64 = la + lb; 
    let l2: f64 = la - lb;
    let v1 = vec![1., -(1. - kp - l1)];
    let v2 = vec![1., -(1. - kp - l2)];
    
    // return normalized eigenvector
    if stable{
        return v1.iter().map(|x| x/(1.+v1[1].powi(2)).sqrt()).collect::<Vec<_>>()
    }
    else{
        return v2.iter().map(|x| x/(1.+v2[1].powi(2)).sqrt()).collect::<Vec<_>>()
    }
}    


/// deviation vector `mLCE=1/n \ln() y_n/y_0`, need to evolve it from start
pub fn deviation_vector(x0: f64, y0: f64, k: f64, t: usize) -> [Vec<f64>; 2]{
    let [x,y] = standard_map(x0, y0, k, t);
    let mut xi1 = vec![0.0; t+1];
    let mut xi2 = vec![0.0; t+1];
    for i in 1..(t){
        xi1[i] = x[i] - x[i-1];
        xi2[i] = y[i] - y[i-1];
    }
    
    let mut m_lce = vec![0.0;t+1];
    for i in 1..t{
        m_lce[i] = m_lce[i-1] + (xi1[i].powi(2) + xi2[i].powi(2)).sqrt().ln();
    }
    // convert integ to Vec<f64>
    let integf64: Vec<f64> = (0..t+1).map(|i| (i as f64)).collect();
    // sum = abs(sum)/i for i in 1..t;
    for i in 1..t{
        m_lce[i] = m_lce[i].abs()/(i as f64);
    }
    [integf64, m_lce]
}
