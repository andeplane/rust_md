mod vector;
mod atom;
use crate::vector::Vector3;
use crate::atom::Atom;

fn create_fcc_basis() -> Vec<Vector3> {
  let mut basis = Vec::with_capacity(4);
  basis.push(Vector3{x: 0.0, y: 0.0, z: 0.0});
  basis.push(Vector3{x: 0.5, y: 0.5, z: 0.0});
  basis.push(Vector3{x: 0.5, y: 0.0, z: 0.5});
  basis.push(Vector3{x: 0.0, y: 0.5, z: 0.5});
  return basis;
}

fn create_fcc_system(num_unit_cells: usize, sigma: f32) -> Vec<Atom> {
  let num_atoms = num_unit_cells * num_unit_cells * num_unit_cells * 4;
  let mut atoms: Vec<Atom> = Vec::with_capacity(num_atoms);
  let basis = create_fcc_basis();

  for i in 0..num_unit_cells {
    for j in 0..num_unit_cells {
      for k in 0..num_unit_cells {
        for basis_vector in &basis {
          let x = (i as f32 + basis_vector.x) * sigma;
          let y = (j as f32 + basis_vector.y) * sigma;
          let z = (k as f32 + basis_vector.z) * sigma;
          
          atoms.push(Atom{
            position: Vector3{x, y, z},
            velocity: Vector3{x: 1.0, y: 1.0, z: 1.0},
            force: Vector3{x: 0.0, y: 0.0, z: 0.0},
            mass: 1.0
          });
        }
      }
    }
  }
  
  return atoms;
}

fn distance(v1: &Vector3, v2: &Vector3) -> f32 {
  let dx = v1.x-v2.x;
  let dy = v1.y-v2.y;
  let dz = v1.z-v2.z;
  (dx*dx + dy*dy + dz*dz).sqrt()
}

fn integrate(atoms: &mut Vec<Atom>) {
  // for atom in atoms {
  //   atom.force.zero();
  // }

  for i in 0..atoms.len() {
    for j in i+1..atoms.len() {
      let dr = distance(&atoms[i].position, &atoms[j].position);
    }
  }
}

fn main() {
  let mut atoms = create_fcc_system(4, 3.405);
  integrate(&mut atoms);
}
