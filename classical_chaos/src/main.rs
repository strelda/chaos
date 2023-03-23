mod module;
use module::{save_map, standard_map, eigenvector,deviation_vector};
use itertools_num::{linspace};

fn main(){
    // set final time t and chaotic parameter k
    let t: usize = 10000; //2000 for standard map, 15 for manifolds
    let k: f64 = 1.;

    ////standard map
    // save_map("standard_map.csv", 
    //         linspace(0.0, 0.5, 10), 
    //         linspace(-0.4, 0.4, 10), 
    //         standard_map,
    //         k, t, false);


    //// stable and unstable manifolds
    //// set x0_all and y0_all on a line along eigenvector from some point
    // let n: usize = 15000;
    // let distance: f64 = 1e-1;
    
    // let x0: f64 = 0.5;
    // let y0: f64 = 0.0;
    // // stable: false -> - direction, stable: true -> + direction
    // let direction = eigenvector(x0, k, true);
    // let x0_all = linspace(x0, x0 + distance*direction[0], n);
    // let y0_all = linspace(y0, y0 + distance*direction[1], n);

    // save_map("standard_map_wiglywogly.csv", 
    //         x0_all,
    //         y0_all,
    //         standard_map,
    //         k, t, true);


    //// deviation vector
    let x0 = 0.2;
    let y0 = 0.0;
    let x01 = 0.3;
    let y01 = 0.0;
    let x02 = 0.55;
    let y02 = 0.0;
    save_map("deviation_vector_k1.csv", 
            linspace(x0,x0, 1), 
            linspace(y0,y0, 1), 
            deviation_vector,
            1., t, false);
    save_map("deviation_vector_k0.1.csv", 
            linspace(x01,x01, 1), 
            linspace(y01,y01, 1), 
            deviation_vector,
            1., t, false);
    save_map("deviation_vector_k0.01.csv", 
                    linspace(x02,x02, 1), 
                    linspace(y02,y02, 1), 
                    deviation_vector,
                    1., t, false);

} 

