use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Alkan {
    Meth,
    Eth,
    Prop,
    But,
    Pent,
    Hex,
    Hept,
    Oct,
    Non,
    Dec,
}

impl Alkan {
    pub fn from(i: u32) -> Result<Alkan> {
        let alk = match i {
            0 => Alkan::Meth,
            1 => Alkan::Eth,
            2 => Alkan::Prop,
            3 => Alkan::But,
            4 => Alkan::Pent,
            5 => Alkan::Hex,
            6 => Alkan::Hept,
            7 => Alkan::Oct,
            8 => Alkan::Non,
            9 => Alkan::Dec,
            _ => return Err(anyhow!("Can't render alkane longer than decan yet.")),
        };
        Ok(alk)
    }
}

impl Alkan {
    pub fn to_enum(str: &String) -> Result<Self> {
        if str.len() < 5 {
            return Err(anyhow! {"to short"});
        }
        let mut str = &str[..str.len() - 2].to_owned();
        let mut s = "".to_string();
        if str.len() == 3 {
            s = format!("_{str}").to_owned();
            str = &s;
        } else if str.len() < 3 {
            return Err(anyhow! {"to short"});
        }
        if str[str.len() - 4..] == *"meth" {
            Ok(Alkan::Meth)
        } else if str[str.len() - 3..] == *"eth" {
            Ok(Alkan::Eth)
        } else if str[str.len() - 4..] == *"prop" {
            Ok(Alkan::Prop)
        } else if str[str.len() - 3..] == *"but" {
            Ok(Alkan::But)
        } else if str[str.len() - 4..] == *"pent" {
            Ok(Alkan::Pent)
        } else if str[str.len() - 3..] == *"hex" {
            Ok(Alkan::Hex)
        } else if str[str.len() - 4..] == *"hept" {
            Ok(Alkan::Hept)
        } else if str[str.len() - 3..] == *"oct" {
            Ok(Alkan::Oct)
        } else if str[str.len() - 3..] == *"non" {
            Ok(Alkan::Non)
        } else if str[str.len() - 3..] == *"dec" {
            Ok(Alkan::Dec)
        } else {
            Err(anyhow!("failed to assign chemical base."))
        }
    }
    pub fn remove_according(str: Vec<char>, alkan: Self) -> Vec<char> {
        let len = match alkan as u8 {
            0 | 2 | 4 | 6 => 6,
            1 | 3 | 5 | 7 | 8 | 9 => 5,
            _ => return vec![],
        };
        let mut str = str;
        for _ in 0..len {
            str.pop();
        }
        str
    }
}

#[derive(Debug, Clone)]
pub struct Side {
    pub base: Alkan,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct AlkanBuilder {
    pub base: Alkan,
    pub sides: Vec<Side>,
    bname: Vec<char>,
}

impl AlkanBuilder {
    pub fn new(name: String) -> Result<Self> {
        let mut name = name;
        name = name.to_lowercase();
        name = name.replace("di", "");
        name = name.replace("tri", "");
        name = name.replace("tetra", "");
        let mut bname: Vec<char> = name.chars().collect();
        bname.retain(|&item| item != '-');
        Ok(Self {
            base: Alkan::to_enum(&name)?,
            sides: vec![],
            bname,
        })
    }
    pub fn trim_base(&mut self) {
        self.bname = Alkan::remove_according(self.bname.clone(), self.base.clone());
    }
    pub fn sort_sides(&mut self) -> Result<()> {
        let mut prev_char = '_';
        let mut chars = vec![];
        for cur_char in self.bname.iter() {
            chars.push(cur_char);
            if prev_char == 'y' && *cur_char == 'l' {
                let base = Alkan::to_enum(&(chars.clone().into_iter().collect()))?;
                let mut chars_useable = vec![];
                for cha in chars.clone().iter() {
                    chars_useable.push(**cha);
                }
                let indices_raw = Alkan::remove_according(chars_useable, base.clone());
                let mut indices = vec![];
                let mut current_num = "".to_string();
                for ch in indices_raw.iter() {
                    if *ch == ',' {
                        let value = match current_num.parse::<u32>() {
                            Ok(v) => v,
                            Err(err) => return Err(anyhow!("{}", err)),
                        };
                        indices.push(value);
                        current_num = "".to_string();
                    } else {
                        current_num.push(*ch);
                    }
                }
                let value = match current_num.parse::<u32>() {
                    Ok(v) => v,
                    Err(err) => {
                        return Err(anyhow!(
                            "{}, (or prefix is greater than tetra please remove prefix the then.)",
                            err
                        ))
                    }
                };
                indices.push(value);
                for index in indices.iter() {
                    self.sides.push(Side {
                        base: base.clone(),
                        index: *index,
                    });
                }
                chars.clear();
            }
            prev_char = *cur_char;
        }
        Ok(())
    }
}
