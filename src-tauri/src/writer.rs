use super::alkan_parser::*;

const C0: &'static str = "C";
const C1: &'static str = "CH";
const C2: &'static str = "CH_2";
const C3: &'static str = "CH_3";

#[derive(Debug, Clone)]
pub struct Atom {
    pub base: &'static str,
    pub mods: Vec<Atom>,
}

#[derive(Debug)]
pub struct AlkanWriter {
    pub structur: Vec<Atom>,
}

impl AlkanWriter {
    pub fn new(albd: &AlkanBuilder) -> Self {
        let mut structur = vec![];
        for _ in 0..(albd.base.clone() as u8) {
            structur.push(Atom {
                base: C2,
                mods: vec![],
            });
        }
        structur.push(Atom {
            base: C3,
            mods: vec![],
        });
        if structur[0].base == C2 {
            structur[0].base = C3;
        }
        Self { structur }
    }
    pub fn add_sides(&mut self, albd: &AlkanBuilder) {
        for side in albd.sides.iter() {
            self.set(side.index as usize - 1, side.base.clone() as u32 + 1);
        }
    }
    pub fn render(&self) -> String {
        if self.structur.len() == 1 {
            return "CH_4".to_string();
        }
        let mut out = "".to_string();
        for c in self.structur.iter() {
            out.push_str(c.base);
            for (i, modi) in c.mods.iter().enumerate() {
                let mut top = false;
                if i % 2 == 0 {
                    top = true;
                }
                out.push_str(Self::render_mod(modi.clone(), top));
            }
            out.push('-');
        }
        out.pop();
        out
    }
    fn set(&mut self, index: usize, add_size: u32) {
        if index == 0 || index >= self.structur.len() {
            return;
        }
        if self.structur[index].base == C3 {
            self.structur[index].base = C2;
        } else if self.structur[index].base == C2 {
            self.structur[index].base = C1;
        } else if self.structur[index].base == C1 {
            self.structur[index].base = C0;
        }

        let out = Self::recursive_add(
            Atom {
                base: C2,
                mods: vec![],
            },
            add_size,
        );
        self.structur[index].mods.push(out);
    }
    pub fn recursive_add(str: Atom, c: u32) -> Atom {
        if c == 1 {
            Atom {
                base: C3,
                mods: vec![],
            }
        } else {
            Atom {
                base: C2,
                mods: vec![Self::recursive_add(str, c - 1)],
            }
        }
    }
    fn render_mod(at: Atom, top: bool) -> &'static str {
        let mut out = "".to_string();
        out.push('(');
        if top {
            out.push_str("-[:90]");
        } else {
            out.push_str("-[:270]");
        }
        out.push_str(at.base);
        for modi in at.mods.iter() {
            out.push_str(Self::render_mod(modi.clone(), top));
        }
        out.push(')');
        out.leak()
    }
}
