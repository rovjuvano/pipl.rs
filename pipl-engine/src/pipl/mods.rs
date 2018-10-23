use ::Molecule;
use ::Name;
use ::Pipl;
use ::Refs;
pub struct Mods<'a, T: 'a> {
    mods: Vec<Mod<T>>,
    pipl: &'a mut Pipl<T>,
}
enum Mod<T> {
    Add(Molecule<T>, Refs),
    Choice(Vec<Molecule<T>>, Refs),
}
impl<'a, T> Mods<'a, T> {
    pub fn new(pipl: &'a mut Pipl<T>) -> Self {
        Mods {
            mods: Vec::new(),
            pipl,
        }
    }
    pub fn add<I: Into<Molecule<T>>>(&mut self, molecule: I, refs: Refs) {
        self.mods.push(Mod::Add(molecule.into(), refs));
    }
    pub fn choice<I: Into<Molecule<T>>>(&mut self, options: Vec<I>, refs: Refs) {
        self.mods.push(Mod::Choice(options.into_iter().map(|x| x.into()).collect(), refs));
    }
    pub fn get_value(&self, name: &Name) -> Option<&T> {
        self.pipl.get_value(name)
    }
    pub fn apply(self) {
        let Mods { mods, pipl } = self;
        for m in mods {
            match m {
                Mod::Add(molecule, refs) => pipl.add_molecule(molecule, refs),
                Mod::Choice(molecules, refs) => pipl.add_choice(molecules, refs),
            }
        }
    }
}
