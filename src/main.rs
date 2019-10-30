use smf::{get_state, my_fmri, PropGetOne, PropertyValue, BoolProp};

fn main() {
    let bp = BoolProp{ inner: true };

    println!("get_value: {:?}", bp.get_value());
    println!("to_string: {:?}", bp.to_string());


    ::std::process::exit(match my_fmri() {
        Err(e) => {
            eprintln!("{}", e);
            1
        }
        Ok(fmri) => {
            println!("{} is {}", fmri, get_state(&fmri).unwrap());
            match PropGetOne(None, &fmri, "general", "action_authorization") {
                Ok(s) => match s {
                    PropertyValue::AString(val) => {
                        println!("{} can be managed by {}", fmri, val.inner);
                    }
                    _ => println!("Unexpected type"),
                },
                Err(e) => println!("No delegated adminstration: {}", e),
            }
            0
        }
    });
}
