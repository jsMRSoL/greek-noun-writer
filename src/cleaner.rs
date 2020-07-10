use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;

// fn main() -> Result<(), Error> {
//     let path = Path::new("lines.txt");
//     clean_file(path)?;
//     Ok(())
// }

pub fn clean_file(p: &Path) -> Result<(), Error> {
    let input = File::open(p)?;
    let buffered = BufReader::new(input);

    let output = File::create("temp-cleaned.txt")?;
    let mut bufout = BufWriter::new(output);

    let re1 = Regex::new(r"[άὰᾶ]").unwrap();
    let re1i = Regex::new(r"[ᾴᾲᾷ]").unwrap();
    let re1u = Regex::new(r"[ἄἂἆ]").unwrap();
    let re1ui = Regex::new(r"[ᾄᾂᾆ]").unwrap();
    let re1a = Regex::new(r"[ἅἃἇ]").unwrap();
    let re1ai = Regex::new(r"[ᾅᾃᾇ]").unwrap();
    let re2 = Regex::new(r"[έὲ]").unwrap();
    let re2u = Regex::new(r"[ἔἒ]").unwrap();
    let re2a = Regex::new(r"[ἕἓ]").unwrap();
    let re3 = Regex::new(r"[ήὴῆ]").unwrap();
    let re3i = Regex::new(r"[ῄῂῇ]").unwrap();
    let re3u = Regex::new(r"[ἤἢἦ]").unwrap();
    let re3ui = Regex::new(r"[ᾔᾒᾖ]").unwrap();
    let re3a = Regex::new(r"[ἥἣἧ]").unwrap();
    let re3ai = Regex::new(r"[ᾕᾓᾗ]").unwrap();
    let re4 = Regex::new(r"[ίὶῖ]").unwrap();
    let re4u = Regex::new(r"[ἴἲἶ]").unwrap();
    let re4a = Regex::new(r"[ἵἳἷ]").unwrap();
    let re5 = Regex::new(r"[όὸ]").unwrap();
    let re5u = Regex::new(r"[ὄὂ]").unwrap();
    let re5a = Regex::new(r"[ὅὃ]").unwrap();
    let re6 = Regex::new(r"[ώὼῶ]").unwrap();
    let re6i = Regex::new(r"[ῴῲῷ]").unwrap();
    let re6u = Regex::new(r"[ὤὢὦ]").unwrap();
    let re6ui = Regex::new(r"[ᾤᾢᾦ]").unwrap();
    let re6a = Regex::new(r"[ὥὣὧ]").unwrap();
    let re6ai = Regex::new(r"[ᾥᾣᾧ]").unwrap();
    let re7 = Regex::new(r"[ύὺῦ]").unwrap();
    let re7u = Regex::new(r"[ὔὒὖ]").unwrap();
    let re7a = Regex::new(r"[ὕὓὗ]").unwrap();

    for line in buffered.lines() {
        let line: String = line?;
        let line = re1.replace_all(&line, "α");
        let line = re1i.replace_all(&line, "ᾳ");
        let line = re1u.replace_all(&line, "ἀ");
        let line = re1ui.replace_all(&line, "ᾀ");
        let line = re1a.replace_all(&line, "ἁ");
        let line = re1ai.replace_all(&line, "ᾁ");
        let line = re2.replace_all(&line, "ε");
        let line = re2u.replace_all(&line, "ἐ");
        let line = re2a.replace_all(&line, "ἑ");
        let line = re3.replace_all(&line, "η");
        let line = re3i.replace_all(&line, "ῃ");
        let line = re3u.replace_all(&line, "ἠ");
        let line = re3ui.replace_all(&line, "ᾐ");
        let line = re3a.replace_all(&line, "ἡ");
        let line = re3ai.replace_all(&line, "ᾑ");
        let line = re4.replace_all(&line, "ι");
        let line = re4u.replace_all(&line, "ἰ");
        let line = re4a.replace_all(&line, "ἱ");
        let line = re5.replace_all(&line, "ο");
        let line = re5u.replace_all(&line, "ὀ");
        let line = re5a.replace_all(&line, "ὁ");
        let line = re6.replace_all(&line, "ω");
        let line = re6i.replace_all(&line, "ῳ");
        let line = re6u.replace_all(&line, "ὠ");
        let line = re6ui.replace_all(&line, "ᾠ");
        let line = re6a.replace_all(&line, "ὡ");
        let line = re6ai.replace_all(&line, "ᾡ");
        let line = re7.replace_all(&line, "υ");
        let line = re7u.replace_all(&line, "ὐ");
        let line = re7a.replace_all(&line, "ὑ");
        let line = line.replace(" ,", ",");
        let line = format!("{}\n", line);
        // println!("{}", line);
        bufout.write(line.as_bytes())?;
    }
    bufout.flush()?;

    Ok(())
}
