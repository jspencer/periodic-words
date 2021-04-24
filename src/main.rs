use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    word_file: Option<PathBuf>,

    #[structopt(short = "w", long = "word")]
    single_word: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let periodic_words = PeriodicWords::new();
    let entries;
    if let Some(file_opt) = opt.word_file {
        let f = File::open(file_opt).expect("Failed to open file");
        entries = BufReader::new(f).split(b'\n');
        for entry in entries {
            let line = String::from_utf8(entry.unwrap()).unwrap();
            periodic_words.print_matches(line, false)
        }
    } else if let Some(word) = opt.single_word {
        periodic_words.print_matches(word, true);
    } else {
        println!("USAGE: `periodic-words -w <single-word>` or `periodic-words -f <file>`");
    }
}

struct Compound(pub Vec<&'static str>);

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for element in &self.0 {
            write!(f, "[{}]", element)?;
        }
        Ok(())
    }
}

struct PeriodicWords {
    periodic_elements: HashMap<&'static str, &'static str>,
}

impl PeriodicWords {
    fn new() -> Self {
        Self {
            periodic_elements: hashmap! {
               "ac" => "Actinium",
               "ag" => "Silver",
               "al" => "Aluminium",
               "am" => "Americium",
               "ar" => "Argon",
               "as" => "Arsenic",
               "at" => "Astatine",
               "au" => "Gold",
               "b" => "Boron",
               "ba" => "Barium",
               "be" => "Beryllium",
               "bh" => "Bohrium",
               "bi" => "Bismuth",
               "bk" => "Berkelium",
               "br" => "Bromine",
               "c" => "Carbon",
               "ca" => "Calcium",
               "cd" => "Cadmium",
               "ce" => "Cerium",
               "cf" => "Californium",
               "cl" => "Chlorine",
               "cm" => "Curium",
               "cn" => "Copernicium",
               "co" => "Cobalt",
               "cr" => "Chromium",
               "cs" => "Caesium",
               "cu" => "Copper",
               "db" => "Dubnium",
               "ds" => "Darmstadtium",
               "dy" => "Dysprosium",
               "er" => "Erbium",
               "es" => "Einsteinium",
               "eu" => "Europium",
               "f" => "Fluorine",
               "fe" => "Iron",
               "fl" => "Flerovium",
               "fm" => "Fermium",
               "fr" => "Francium",
               "ga" => "Gallium",
               "gd" => "Gadolinium",
               "ge" => "Germanium",
               "h" => "Hydrogen",
               "he" => "Helium",
               "hf" => "Hafnium",
               "hg" => "Mercury",
               "ho" => "Holmium",
               "hs" => "Hassium",
               "i" => "Iodine",
               "in" => "Indium",
               "ir" => "Iridium",
               "k" => "Potassium",
               "kr" => "Krypton",
               "la" => "Lanthanum",
               "li" => "Lithium",
               "lr" => "Lawrencium",
               "lu" => "Lutetium",
               "lv" => "Livermorium",
               "mc" => "Moscovium",
               "md" => "Mendelevium",
               "mg" => "Magnesium",
               "mn" => "Manganese",
               "mo" => "Molybdenum",
               "mt" => "Meitnerium",
               "n" => "Nitrogen",
               "na" => "Sodium",
               "nb" => "Niobium",
               "nd" => "Neodymium",
               "ne" => "Neon",
               "nh" => "Nihonium",
               "ni" => "Nickel",
               "no" => "Nobelium",
               "np" => "Neptunium",
               "o" => "Oxygen",
               "og" => "Oganesson",
               "os" => "Osmium",
               "p" => "Phosphorus",
               "pa" => "Protactinium",
               "pb" => "Lead",
               "pd" => "Palladium",
               "pm" => "Promethium",
               "po" => "Polonium",
               "pr" => "Praseodymium",
               "pt" => "Platinum",
               "pu" => "Plutonium",
               "ra" => "Radium",
               "rb" => "Rubidium",
               "re" => "Rhenium",
               "rf" => "Rutherfordium",
               "rg" => "Roentgenium",
               "rh" => "Rhodium",
               "rn" => "Radon",
               "ru" => "Ruthenium",
               "s" => "Sulfur",
               "sb" => "Antimony",
               "sc" => "Scandium",
               "se" => "Selenium",
               "sg" => "Seaborgium",
               "si" => "Silicon",
               "sm" => "Samarium",
               "sn" => "Tin",
               "sr" => "Strontium",
               "ta" => "Tantalum",
               "tb" => "Terbium",
               "tc" => "Technetium",
               "te" => "Tellurium",
               "th" => "Thorium",
               "ti" => "Titanium",
               "tl" => "Thallium",
               "tm" => "Thulium",
               "ts" => "Tennessine",
               "u" => "Uranium",
               "v" => "Vanadium",
               "w" => "Tungsten",
               "xe" => "Xenon",
               "y" => "Yttrium",
               "yb" => "Ytterbium",
               "zn" => "Zinc",
               "zr" => "Zirconium",
            },
        }
    }

    fn gen_compound(&self, s: &str) -> Option<Vec<&'static str>> {
        assert!(s.is_ascii());
        let len = s.len();
        let mut sol = vec![None; s.len() + 1];
        // init recurrence-relation base cases
        // end of the string is a valid solution
        sol[len] = Some(vec![]);
        if let Some(&elem) = self.periodic_elements.get(&s[(len - 1)..]) {
            sol[len - 1] = Some(vec![elem])
        }

        for i in (0..(len - 1)).rev() {
            let fst_sol = &sol[i + 1];
            let snd_sol = &sol[i + 2];
            let fst = self.periodic_elements.get(&s[i..(i + 1)]);
            let snd = self.periodic_elements.get(&s[i..(i + 2)]);
            match (fst_sol, fst, snd_sol, snd) {
                (Some(fst_sol), Some(&elem_one), Some(snd_sol), Some(&elem_two)) => {
                    // favor shorter compounds
                    sol[i] = Some(if fst_sol.len() < snd_sol.len() {
                        appended_vec(fst_sol, elem_one)
                    } else {
                        appended_vec(snd_sol, elem_two)
                    })
                }
                (Some(prev_sol), Some(&elem), _, _) | (_, _, Some(prev_sol), Some(&elem)) => {
                    sol[i] = Some(appended_vec(prev_sol, elem))
                }
                // item already init to None, continue
                _ => (),
            }
        }

        sol[0]
            .take()
            .map(|s| s.into_iter().rev().collect::<Vec<_>>())
    }

    fn print_matches(&self, line: String, show_misses: bool) {
        let line = line.to_ascii_lowercase();
        let maybe_compound = self.gen_compound(line.as_str());
        if let Some(compound) = maybe_compound {
            let compound = Compound(compound);
            println!("{} {}", line, compound);
        } else if show_misses {
            println!(
                "\"{}\" can't be written with the periodic table until we discover more elements.",
                line
            );
        }
    }
}

#[inline]
fn appended_vec(base_vec: &Vec<&'static str>, elem: &'static str) -> Vec<&'static str> {
    let mut vec = base_vec.clone();
    vec.push(elem);
    vec
}

#[cfg(test)]
mod tests {
    use super::PeriodicWords;

    #[test]
    fn dyn_chemical_compound_works() {
        let test = PeriodicWords::new();
        assert_eq!(
            test.gen_compound("farts"),
            Some(vec!["Fluorine", "Argon", "Tennessine"])
        );
        assert_eq!(
            test.gen_compound("fun"),
            Some(vec!["Fluorine", "Uranium", "Nitrogen"])
        );
        assert_eq!(
            test.gen_compound("brew"),
            Some(vec!["Boron", "Rhenium", "Tungsten"])
        );
        assert_eq!(test.gen_compound("ew"), None);
        assert_eq!(
            test.gen_compound("carbon"),
            Some(vec!["Calcium", "Rubidium", "Oxygen", "Nitrogen"])
        );
        assert_eq!(
            test.gen_compound("xenon"),
            Some(vec!["Xenon", "Nobelium", "Nitrogen"])
        );
        assert_eq!(
            test.gen_compound("silver"),
            Some(vec!["Silicon", "Livermorium", "Erbium"])
        );
        assert_eq!(
            test.gen_compound("oganesson"),
            Some(vec![
                "Oxygen", "Gallium", "Neon", "Sulfur", "Sulfur", "Oxygen", "Nitrogen"
            ])
        );
        assert_eq!(
            test.gen_compound("copper"),
            Some(vec!["Cobalt", "Phosphorus", "Phosphorus", "Erbium"])
        );
        assert_eq!(
            test.gen_compound("silicon"),
            Some(vec!["Silicon", "Lithium", "Cobalt", "Nitrogen"])
        );
        assert_eq!(test.gen_compound("tin"), Some(vec!["Titanium", "Nitrogen"]));
        assert_eq!(
            test.gen_compound("dynamite"),
            Some(vec![
                "Dysprosium",
                "Nitrogen",
                "Americium",
                "Iodine",
                "Tellurium"
            ])
        );
        assert_eq!(test.gen_compound("zummer"), None);
    }
}
