use smf::{my_fmri, get_state};

fn main() {
    ::std::process::exit(match my_fmri() {
        Err(e) => {
            eprintln!("No FMRI found: {}", e);
            1
        },
        Ok(fmri) => {
            println!("{} is {}", fmri, get_state(&fmri).unwrap());
            0
        }
    });
}
