mod funcs;

use clap::{App, Arg};
use csv::{ByteRecord, Reader, Writer};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Deserialize, Debug, Clone)]
struct Record {
    #[serde(rename = "nom")]
    nom_sing: String,
    #[serde(rename = "gen")]
    gen_sing: String,
    gender: String,
}

#[derive(Debug)]
struct Noun {
    nom_sing: String,
    gen_sing: String,
    stem: String,
    gender: Gender,
    // regularity: Regularity,
    nountype: NounType,
    declined: Option<Vec<String>>,
    declined_w_article: Option<Vec<String>>,
}

impl Noun {
    fn new(ns: String, gs: String, g: Gender, n: NounType) -> Self {
        let mut gs_string = gs.clone();
        let stm = match &n {
            &NounType::Polis | &NounType::Genos => {
                gs_string.pop();
                gs_string.pop();
                gs_string.pop();
                gs_string
            }
            _ => {
                gs_string.pop();
                gs_string.pop();
                gs_string
            }
        };
        Self {
            nom_sing: ns,
            gen_sing: gs,
            stem: stm,
            gender: g,
            // regularity: r,
            nountype: n,
            declined: None,
            declined_w_article: None,
        }
    }

    fn decline(&mut self) {
        let endings: Vec<&str> = self.nountype.get_endings();
        let articles: Vec<&str> = self.gender.get_article();
        let combined = articles.iter().zip(endings.iter());
        let mut declined: Vec<String> = Vec::new();
        let mut declined_w_article: Vec<String> = Vec::new();
        let mut form: String;
        for (art, end) in combined {
            form = format!("{}{}", self.stem, end);
            match self.nountype {
                NounType::Phulax => {
                    form = form.replace("κσ", "ξ");
                    form = form.replace("κτσ", "ξ");
                    form = form.replace("δσ", "σ");
                    form = form.replace("τσ", "σ");
                }
                NounType::Cheimon => form = form.replace("νσ", "σ"),
                NounType::Geron => form = form.replace("οντσι", "ουσι"),
                NounType::Gigas => form = form.replace("αντσι", "ασι"),
                NounType::Soma => form = form.replace("ατσι", "ασι"),
                _ => (),
            };
            declined.push(format!("{}", form));
            declined_w_article.push(format!("{} {}", art, form));
        }
        match self.nountype {
            NounType::Phulax | NounType::Cheimon | NounType::Geron | NounType::Gigas => {
                declined[0] = format!("{}", self.nom_sing);
                declined_w_article[0] = format!("{} {}", articles[0], self.nom_sing);
            }
            NounType::Soma | NounType::Genos => {
                declined[0] = format!("{}", self.nom_sing);
                declined[1] = format!("{}", self.nom_sing);
                declined_w_article[0] = format!("{} {}", articles[0], self.nom_sing);
                declined_w_article[1] = format!("{} {}", articles[1], self.nom_sing);
            }
            _ => (),
        }
        self.declined = Some(declined);
        self.declined_w_article = Some(declined_w_article);
    }

    fn print_noun(&self) {
        let mut noun = String::new();
        for part in self.declined.as_ref().unwrap() {
            noun = format!("{}, {}", noun, part);
        }
        println!("{}", &noun[2..]);
    }

    fn print_noun_w_article(&self) {
        let mut noun = String::new();
        for part in self.declined_w_article.as_ref().unwrap() {
            noun = format!("{}, {}", noun, part);
        }
        println!("{}", &noun[2..]);
    }
}

#[derive(Debug)]
enum NounType {
    Chora,
    Time,
    Mousa,
    Krites,
    Neanias,
    Logos,
    Doron,
    Phulax,
    Cheimon,
    Geron,
    Gigas,
    Basileus,
    Genos,
    Soma,
    Polis,
    Ichthus,
}

impl NounType {
    fn get_endings(&self) -> Vec<&str> {
        match self {
            NounType::Chora => vec!["α", "αν", "ας", "ᾳ", "αι", "ας", "ων", "αις"],
            NounType::Time => vec!["η", "ην", "ης", "ῃ", "αι", "ας", "ων", "αις"],
            NounType::Mousa => vec!["α", "αν", "ης", "ῃ", "αι", "ας", "ων", "αις"],
            NounType::Krites => vec!["ης", "ην", "ου", "ῃ", "αι", "ας", "ων", "αις"],
            NounType::Neanias => vec!["ας", "αν", "ου", "ᾳ", "αι", "ας", "ων", "αις"],
            NounType::Logos => vec!["ος", "ον", "ου", "ῳ", "οι", "ους", "ων", "οις"],
            NounType::Doron => vec!["ον", "ον", "ου", "ῳ", "α", "α", "ων", "οις"],
            NounType::Phulax => vec!["", "α", "ος", "ι", "ες", "ας", "ων", "σι"],
            NounType::Cheimon => vec!["", "α", "ος", "ι", "ες", "ας", "ων", "σι"],
            NounType::Geron => vec!["", "α", "ος", "ι", "ες", "ας", "ων", "σι"],
            NounType::Gigas => vec!["", "α", "ος", "ι", "ες", "ας", "ων", "σι"],
            NounType::Basileus => vec!["υς", "α", "ως", "ι", "ις", "ας", "ων", "υσι"],
            NounType::Genos => vec!["", "", "ους", "ει", "η", "η", "ων", "εσι"],
            NounType::Polis => vec!["ις", "ιν", "εως", "ει", "εις", "εις", "εων", "εσι"],
            NounType::Ichthus => vec!["ς", "ν", "ος", "ι", "εις", "εις", "ων", "σι"],
            NounType::Soma => vec!["", "", "ος", "ι", "α", "α", "ων", "σι"],
        }
    }

    // fn clean_declined(&self, v: Vec<&str>) -> Vec<&str> {
    //     unimplemented!();
    // }
}

#[derive(Debug)]
enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

impl Gender {
    fn get_article(&self) -> Vec<&str> {
        match self {
            Gender::Masculine => vec!["ὁ", "τον", "του", "τῳ", "οἱ", "τους", "των", "τοις"],
            Gender::Feminine => vec!["ἡ", "την", "της", "τῃ", "αἱ", "τας", "των", "ταις"],
            Gender::Neuter => vec!["το", "το", "του", "τῳ", "τα", "τα", "των", "τοις"],
        }
    }
}

// enum Regularity {
//     Regular,
//     Irregular,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Greek noun decliner")
        .about(
            "
This program declines Greek nouns, printing to stdout by default.
The noun is produced without the article by default. Change this by
supplying a flag. You can write output to csv format. In this case,
the noun is given first without the article and then with it.",
        )
        .arg(
            Arg::with_name("infile")
                .help("File to read from")
                .index(1)
                .required_unless("from-str")
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("from-str")
                .help("String to read from.")
                .short("s")
                .long("from-str")
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outfile")
                .help("File to write to.")
                .short("o")
                .long("outfile")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("with-article")
                .help("Print with article.")
                .short("w")
                .long("with-article"),
        )
        .get_matches();

    let infile_supplied: bool = matches.is_present("infile");
    let source = if infile_supplied {
        let infile_name = matches.value_of("infile").unwrap();
        if !funcs::check_file(infile_name) {
            eprintln!("File {} contains accents. Please remove them.", infile_name);
            process::exit(1);
        }
        let contents = fs::read_to_string(infile_name).expect("Could not read infile.");
        contents
    } else {
        let contents = matches.value_of("from-str").unwrap();
        contents.to_string()
    };

    if matches.is_present("outfile") {
        let outfile_name = matches.value_of("outfile").unwrap_or("output.csv");
        write_to_csv(&source, outfile_name, infile_supplied)?;
    } else {
        let vb_vec: Vec<&str> = source.split(",").collect();
        if vb_vec.len() > 3 {
            eprintln!("Too many parts supplied. Did you mean to include an 'outfile' argument?");
            process::exit(1);
        }
        let ns = vb_vec[0].trim().to_string();
        let gs = vb_vec[1].trim().to_string();
        let gen = vb_vec[2];
        let ntype: NounType = get_type(&ns, &gs)?;
        let gender: Gender = get_gender(&gen)?;
        let mut noun: Noun = Noun::new(ns, gs, gender, ntype);
        noun.decline();
        if matches.is_present("with-article") {
            noun.print_noun_w_article();
        } else {
            noun.print_noun();
        }
    }
    Ok(())
}

fn write_to_csv(source: &str, outfile: &str, hdrs_from_file: bool) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_reader(source.as_bytes());
    if !hdrs_from_file {
        rdr.set_byte_headers(ByteRecord::from(vec!["nom", "gen", "gender"]));
    }

    let mut nouns: Vec<Noun> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;
        // let record: Record = match result {
        //     Ok(r) => r,
        //     Err(e) => {
        //         eprintln!("Bad record? {}", e);
        //         continue;
        //     } // Err(e) => return Err(From::from(e)),
        // };
        let ns = record.nom_sing.trim();
        let gs = record.gen_sing.trim();
        let gen = record.gender.trim();
        // let ntype: NounType = get_type(&ns, &gs)?;
        // let ntype: NounType;
        let ntype: NounType = match get_type(&ns, &gs) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        // let gender: Gender = get_gender(&gen)?;
        let gender: Gender = match get_gender(&gen) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Error with {}: {}", ns, e);
                continue;
            }
        };
        let mut noun: Noun = Noun::new(ns.to_string(), gs.to_string(), gender, ntype);
        noun.decline();
        nouns.push(noun);
    }

    let mut wtr = Writer::from_path(outfile)?;
    for noun in nouns {
        let declined = &noun.declined;
        wtr.write_record(declined.as_ref().unwrap())?;
        let declined_w_article = &noun.declined_w_article;
        wtr.write_record(declined_w_article.as_ref().unwrap())?;
        // &noun.print_noun();
        // &noun.print_noun_w_article();
    }
    wtr.flush()?;
    Ok(())
}

fn get_type(ns: &str, gs: &str) -> Result<NounType, String> {
    match (ns.trim(), gs.trim()) {
        // First declension feminines
        (p0, p1) if p0.ends_with("η") && p1.ends_with("ης") => Ok(NounType::Time),
        (p0, p1) if p0.ends_with("α") && p1.ends_with("ας") => Ok(NounType::Chora),
        (p0, p1) if p0.ends_with("α") && p1.ends_with("ης") => Ok(NounType::Mousa),
        (p0, p1) if p0.ends_with("ης") && p1.ends_with("ου") => Ok(NounType::Krites),
        (p0, p1) if p0.ends_with("ας") && p1.ends_with("ου") => Ok(NounType::Neanias),
        // Second declension
        (p0, p1) if p0.ends_with("ος") && p1.ends_with("ου") => Ok(NounType::Logos),
        (p0, p1) if p0.ends_with("ον") && p1.ends_with("ου") => Ok(NounType::Doron),
        (p0, p1) if p0.ends_with("οι") && p1.ends_with("ων") => Ok(NounType::Logos),
        // Third declension
        (p0, p1) if p0.ends_with("ων") && p1.ends_with("οντος") => Ok(NounType::Geron),
        (p0, p1) if p0.ends_with("ας") && p1.ends_with("αντος") => Ok(NounType::Gigas),
        (p0, p1) if p0.ends_with("α") && p1.ends_with("ατος") => Ok(NounType::Soma),
        (p0, p1) if p0.ends_with("τα") && p1.ends_with("ατων") => Ok(NounType::Soma),
        (p0, p1) if p0.ends_with("α") && p1.ends_with("ων") => Ok(NounType::Doron),
        (p0, p1) if p0.ends_with("ος") && p1.ends_with("ους") => Ok(NounType::Genos),
        (p0, p1) if p0.ends_with("ευς") && p1.ends_with("εως") => Ok(NounType::Basileus),
        (p0, p1) if p0.ends_with("ις") && p1.ends_with("εως") => Ok(NounType::Polis),
        (p0, p1) if p0.ends_with("υς") && p1.ends_with("υος") => Ok(NounType::Ichthus),
        (_p0, p1) if p1.ends_with("νος") => Ok(NounType::Cheimon),
        (_p0, p1) if p1.ends_with("ος") => Ok(NounType::Phulax),
        (p0, p1) => Err(format!("[{}, {}] is not a recognised noun type.", p0, p1)),
    }
}

fn get_gender(s: &str) -> Result<Gender, String> {
    match s.trim() {
        g if g == "ἡ" => Ok(Gender::Feminine),
        g if g == "ὁ" => Ok(Gender::Masculine),
        g if g == "το" => Ok(Gender::Neuter),
        g if g == "αἱ" => Ok(Gender::Feminine),
        g if g == "οἱ" => Ok(Gender::Masculine),
        g if g == "τα" => Ok(Gender::Neuter),
        _ => Err(format!("\'{}\' is not recognised as one of ὁ, ἡ, το.", s)),
    }
}
