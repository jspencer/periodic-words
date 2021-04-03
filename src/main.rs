use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use std::collections::HashMap;
use maplit::hashmap;

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

struct Compound(pub Vec<String>);

impl Compound {
    fn push(&mut self, value: String) {
        self.0.push(value)
    }
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, element| {
            result.and_then(|_| write!(f, "[{}]", element))
        })
    }
}

struct PeriodicWords {
    periodic_elements: HashMap<&'static str, &'static str>
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
    }
        }
    }

    fn print_matches(&self, line: String, show_misses: bool) {
        let mut full_match = true;
        let mut prev_was_double = false;
        let mut compound = Compound(Vec::new());
        for n in 0..line.to_lowercase().chars().count() {
            if prev_was_double {
                prev_was_double = false;
                continue;
            }
            let this_char = line.chars().nth(n).unwrap_or('?');
            let next_char = line.chars().nth(n + 1).unwrap_or('*');
            let mut the_double: String = "".to_string();
            the_double.push(this_char);
            the_double.push(next_char);
            let mut the_single: String = "".to_string();
            the_single.push(this_char);

            if let Some(single) = self.periodic_elements.get(the_single.as_str()) {
                compound.push(single.to_string());
            } else if let Some(double) = self.periodic_elements.get(the_double.as_str()) {
                compound.push(double.to_string());
                prev_was_double = true;
            } else {
                full_match = false;
            }
        }
        if full_match {
            println!("{} {}", line, compound);
        } else if show_misses {
            println!("\"{}\" can't be written with the periodic table until we discover more elements.", line);
        }
    }
}
