use ::Molecule;
use ::Pipl;
use ::Refs;
pub struct Mods {
    mods: Vec<(Molecule, Refs)>,
}
impl Mods {
    pub fn new() -> Mods {
        Mods {
            mods: Vec::new(),
        }
    }
    pub fn add<T: Into<Molecule>>(&mut self, molecule: T, refs: Refs) {
        self.mods.push((molecule.into(), refs));
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for (molecule, refs) in self.mods {
            pipl.add_molecule(molecule, refs);
        }
    }
}
