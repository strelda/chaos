use std::f64::consts::PI;
use std::io::Write;


/// A quantum well object with sides a[i]. The length of a sets up the dimension of the well.
/// ## Arguments:
/// * `sides` - A vector of the lengths of the sides of the well
pub struct Well {
   sides: Vec<f64>,
}

impl Well {
   pub fn new(sides: Vec<f64>) -> Well {
      Well {
         sides,
      }
   }
   /// Calculates the energy of a quantum well with quantum numbers `n`.
   /// ## Arguments:
   /// * `n` - A vector of quantum numbers
   /// ## Example:
   /// `let well2d: Well = Well::new(vec![a,b])`
   pub fn h(&self, n:Vec<i64>) -> f64 {
      0.5 * n.iter()
             .zip(self.sides.iter())
             .map(|(ni, ai)| 
                     (PI * *ni as f64 / *ai).powi(2))
             .sum::<f64>()
   }

   /// Calculate all energies of a quantum well up to `emax` and write them to a file called `filename`.
   /// ## Arguments:
   /// * `emax` - The maximum energy to calculate
   /// * `filename` - The name of the file to write the energies to
   /// ## Example:
   /// `well2d.calculate_energies(emax, "generated_data/well_energies.txt");`
   pub fn calculate_energies(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
      let dim = self.sides.len();

      match dim {
         2 => self.calculate_energies_2d(emax, filename),
         3 => self.calculate_energies_3d(emax, filename),
         _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Dimension not supported")),
      }
   }

   fn calculate_energies_2d(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
      let mut n: Vec<i64> = vec![1,1];
      let mut energy: f64 = 0.;
      let mut file = std::fs::File::create(filename)?;

      while energy < emax {
         while energy < emax {
            energy = self.h(n.clone());
            writeln!(file, "{}", energy)?;      
            n[1] += 1;
         }
         n[0] += 1;
         energy = self.h(n.clone());
      }
         
      Ok(())
   }
   fn calculate_energies_3d(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
      let mut n: Vec<i64> = vec![1,1,1];
      let mut energy: f64 = 0.;
      let mut file = std::fs::File::create(filename)?;

      while energy < emax {
         while energy < emax {
            while energy < emax {
               energy = self.h(n.clone());
               writeln!(file, "{}", energy)?;      
               n[2] += 1;
               
            }
            n[1] += 1;
            n[2] = 1;
            energy = self.h(n.clone());
         }
         n[0] += 1;
         n[1] = 1;
         energy = self.h(n.clone());
      }
         
      Ok(())
   }

}




