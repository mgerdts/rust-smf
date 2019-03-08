use smf::my_fmri;

fn main() {
    ::std::process::exit(match my_fmri() {
        Err(e) => {
            eprintln!("No FMRI found: {}", e);
            1
        },
        Ok(fmri) => {
            println!("{}", fmri);
            0
        }
    });
}
