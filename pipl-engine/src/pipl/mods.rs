use ::Molecule;
use ::Pipl;
use ::Refs;
pub struct Mods {
    mods: Vec<Mod>,
}
enum Mod {
    Add(Molecule, Refs),
    Remove(Molecule, Refs),
}
impl Mods {
    pub fn new() -> Mods {
        Mods {
            mods: Vec::new(),
        }
    }
    pub fn add<T: Into<Molecule>>(&mut self, molecule: T, refs: Refs) {
        self.mods.push(Mod::Add(molecule.into(), refs));
    }
    pub fn remove<T: Into<Molecule>>(&mut self, molecule: T, refs: Refs) {
        self.mods.push(Mod::Remove(molecule.into(), refs));
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for m in self.mods {
            match m {
                Mod::Add(molecule, refs) => pipl.add_molecule(molecule, refs),
                Mod::Remove(molecule, refs) => pipl.remove_molecule(molecule, refs),
            }
        }
    }
}
