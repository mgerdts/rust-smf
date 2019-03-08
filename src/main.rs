use smf::my_fmri;

fn main() {
    match my_fmri() {
        Some(fmri) => println!("FMRI: {}", fmri),
        None => println!("Nada"),
    }
}
