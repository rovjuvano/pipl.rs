use ::Molecule;
use ::Pipl;
use ::Refs;
pub struct Mods<T> {
    mods: Vec<Mod<T>>,
}
enum Mod<T> {
    Add(Molecule<T>, Refs<T>),
    Choice(Vec<Molecule<T>>, Refs<T>),
}
impl<T> Mods<T> {
    pub fn new() -> Mods<T> {
        Mods {
            mods: Vec::new(),
        }
    }
    pub fn add<I: Into<Molecule<T>>>(&mut self, molecule: I, refs: Refs<T>) {
        self.mods.push(Mod::Add(molecule.into(), refs));
    }
    pub fn choice<I: Into<Molecule<T>>>(&mut self, options: Vec<I>, refs: Refs<T>) {
        self.mods.push(Mod::Choice(options.into_iter().map(|x| x.into()).collect(), refs));
    }
    pub fn apply(self, pipl: &mut Pipl<T>) {
        for m in self.mods {
            match m {
                Mod::Add(molecule, refs) => pipl.add_molecule(molecule, refs),
                Mod::Choice(molecules, refs) => pipl.add_choice(molecules, refs),
            }
        }
    }
}
