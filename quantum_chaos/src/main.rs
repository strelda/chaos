mod module;
use std::f64::consts::PI;
use std::io::Write;
use module::{well_energy};

fn calculate_well_energies(a: f64, b: f64, emax: f64) -> Result<(), std::io::Error> {
    let mut n1: i64 = 1;
    let mut n2: i64 = 1;
    let mut energy: f64 = 0.;
    let mut file = std::fs::File::create("generated_data/well_energies.txt")?;

    while energy < emax {
        energy = 0.0;
        while energy < emax {
            energy = well_energy(n1, n2, a, b);
            writeln!(file, "{}", energy)?;
            n2 += 1;
        }
        n2 = 1;
        n1 += 1;
    }
    Ok(())
}

fn calculate_well_energy_differences(a: f64, b: f64, emax: f64) -> Result<(), std::io::Error> {
    let mut n1: i64 = 1;
    let mut n2: i64 = 1;
    let mut energy1: f64 = 0.;
    let mut energy2: f64 = 0.;
    let mut file = std::fs::File::create("generated_data/ps.txt")?;

    while energy1 < emax {
        energy1 = well_energy(n1, n1, a, b);
        n2 = n1;
        energy2 = 0.;
        while energy2 < emax {
            energy2 = well_energy(n1, n2 + 1, a, b);
            let s: f64 = energy2 - energy1;

            writeln!(file, "{}", s)?;
            n2 += 1;
            energy1 = energy2;
        }
        n1 += 1;
    }
    Ok(())
}




fn main() {
    // set well parameters
    let a: f64 = (PI / 3.0).sqrt();
    let b: f64 = 1.0;
    // set the criteria for the energy histogram
    let emax: f64 = 1e6;

    // generate the energy histogram data
    match calculate_well_energies(a, b, emax) {
        Ok(()) => println!("Energies calculated successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

    // generate the energy differences data
    match calculate_well_energy_differences(a, b, emax) {
        Ok(()) => println!("Energy differences calculated successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
    
}
