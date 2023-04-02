mod module;
use std::f64::consts::PI;
use std::f64::consts::E;
use module::{*};


/// Calculate the 2D well energies and energy differences
/// ## Arguments:
/// * `a` - The length of the first side of the well
/// * `b` - The length of the second side of the well
/// * `emax` - The maximum energy to calculate
/// * `filename` - The name of the file to write the energies to
/// ## Example:
/// `run_2d((PI / 3.0).sqrt() , 1.0, 1e6, "generated_data/2d/energies.txt", "generated_data/2d/endif.txt");`
fn run_2d(a: f64, b: f64, emax: f64, filename_ene: &str) {
   let well2d = Well::new(vec![a,b]);

   match well2d.calculate_energies(emax, filename_ene) {
      Ok(()) => println!("Energies for 2D well calculated successfully."),
      Err(e) => eprintln!("Error occurred in well2d.calculate_energies(): {}", e),
   }

}

/// Calculate the 3D well energies and energy differences
/// ## Arguments:
/// * `a` - The length of the first side of the well
/// * `b` - The length of the second side of the well
/// * `c` - The length of the third side of the well
/// * `emax` - The maximum energy to calculate
/// * `filename_ene` - The name of the file to write the energies to
/// * `filename_dif` - The name of the file to write the energy differences to
/// ## Example:
/// `run_3d(1.0, (PI / 3.0).sqrt() , (E.powi(3)/21.0).sqrt(), 1e6, "generated_data/2d/energies.txt", "generated_data/2d/endif");`
fn run_3d(a: f64, b: f64, c: f64, emax: f64, filename_ene: &str) {
   let well3d = Well::new(vec![a,b,c]);

   match well3d.calculate_energies(emax, filename_ene) {
      Ok(()) => println!("Energies for 3D well calculated successfully."),
      Err(e) => eprintln!("Error occurred in well3d.calculate_energies(): {}", e),
   }
}

fn main() {
 
    run_2d((PI / 3.0).sqrt() , 1.0, 1e6, "generated_data/2d/energies.txt");

    run_3d(1.0, (PI / 3.0).sqrt() , (E.powi(3)/21.0).sqrt(), 1e5, "generated_data/3d/energies.txt");

}
