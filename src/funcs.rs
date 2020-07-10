use regex::Regex;
use std::fs;

pub fn check_file(path: &str) -> bool {
    let contents = fs::read_to_string(path).expect(&format!("Unable to read {}.", path));
    // unshifted = accents: ; = acute e.g. ά ' = grave e.g. ὰ
    // shifted = breathings: : = smooth e.g. ἀ @ = rough e.g. ἁ
    // you must press the unshifted key first!
    // ] = iota subscript [ = circumflex. If both, ][
    let re = Regex::new(r"[άἄἅὰἂἃᾴᾲᾷᾶέὲἔἒἕἓήἤἥὴἢἣῆῄῂῇίἴἵὶἲἳόὄὅὸὂὃώὤὥὼὣὢῶῴῲῷύὔὕὺὒὓ]").unwrap();
    if re.is_match(&contents) {
        false
    } else {
        true
    }
}
