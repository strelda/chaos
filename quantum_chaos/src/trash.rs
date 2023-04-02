
impl Well {
    ...


/// Calculate all energy differences of a quantum well up to `emax` and write them to a file called `filename`.
/// ## Arguments:
/// * `emax` - The maximum energy to calculate
/// * `filename` - The name of the file to write the energies to
/// ## Example:
/// `well2d.calculate_energy_differences(emax, "generated_data/ps.txt");`
 pub fn calculate_energy_differences(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
    let dim = self.sides.len();
    
    match dim {
       2 => self.calculate_energy_differences_2d(emax, filename),
       3 => self.calculate_energy_differences_3d(emax, filename),
       _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Dimension not supported")),
    }
  }
  fn calculate_energy_differences_2d(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
    let mut n: Vec<i64> = vec![1; self.sides.len()];
    let mut energy1: f64 = 0.;
    let mut file = std::fs::File::create(filename)?;

    while energy1 < emax {
       energy1 = self.h(n.clone());
       n[0] += 1;
       let mut energy2 = 0.;
       while energy2 < emax {
          energy2 = self.h(n.clone());
          let s: f64 = energy2 - energy1;

          writeln!(file, "{}", s)?;
          n[0] += 1;
          energy1 = energy2;
       }
       n[0] = 1;
       n[1] += 1;
    }
    Ok(())
 }
 fn calculate_energy_differences_3d(&self, emax: f64, filename: &str) -> Result<(), std::io::Error> {
    let mut n: Vec<i64> = vec![1; self.sides.len()];
    let mut energy1: f64 = 0.;
    let mut file = std::fs::File::create(filename)?;

    while energy1 < emax {
       energy1 = self.h(n.clone());
       n[0] += 1;
       let mut energy2 = 0.;
       while energy2 < emax {
          energy2 = self.h(n.clone());
          n[0] += 1;
          let mut energy3 = 0.;
          while energy3 < emax {
             energy3 = self.h(n.clone());
             let s: f64 = energy3 - energy2;

             writeln!(file, "{}", s)?;
             n[0] += 1;
             energy2 = energy3;
          }
          n[0] = 1;
          n[1] += 1;
          energy1 = energy2;
       }
       n[0] = 1;
       n[1] = 1;
       n[2] += 1;
    }
    Ok(())
 }

}