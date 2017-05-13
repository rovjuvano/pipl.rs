use ::Molecule;
use ::Pipl;
use ::Refs;
pub struct Mods {
    mods: Vec<Mod>,
}
enum Mod {
    Add(Molecule, Refs),
    Choice(Vec<Molecule>, Refs),
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
    pub fn choice<T: Into<Molecule>>(&mut self, options: Vec<T>, refs: Refs) {
        self.mods.push(Mod::Choice(options.into_iter().map(|x| x.into()).collect(), refs));
    }
    pub fn apply(self, pipl: &mut Pipl) {
        for m in self.mods {
            match m {
                Mod::Add(molecule, refs) => pipl.add_molecule(molecule, refs),
                Mod::Choice(molecules, refs) => pipl.add_choice(molecules, refs),
            }
        }
    }
}
