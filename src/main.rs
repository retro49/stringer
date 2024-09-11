extern crate stringer;

fn stringer_write(path: String, conf: stringer::config::StringerConfig, out: Option<String>) {
    let file = std::fs::File::open(path);
    match file {
        Ok(_) => {}
        Err(_) => {
            println!("cannot open file");
            return;
        },
    };

    let mut file = file.unwrap();
    let result = stringer::Stringer::new(&mut file);
    match result {
        Ok(mut r) => {
            r.set_config(conf);
            r.read_strings();
            match out {
                Some(p) => {
                    let f = std::fs::File::create_new(p);
                    match f {
                        Ok(mut f) => {
                            match stringer::writer::write(&mut f, &r.results) {
                                Ok(_) => {}
                                Err(_) => {
                                    println!("unable to write data to file");
                                }
                            };
                        }
                        Err(_) => {}
                    };
                },
                None => {
                    let mut stdout = std::io::stdout();
                    let _lock = stdout.lock();
                    match stringer::writer::write(&mut stdout, &r.results) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("unable to write data");
                        }
                    };
                    // drop(lock);
                }
            }
        }
        Err(_) => {}
    }

}

fn main() {
    let args = stringer::args::Args::parse_args();
    match args {
        Ok(arg) => {
            let output = &arg.output;
            let output = match output {
                Some(s)  => { Some(s.clone()) },
                None =>  { None },
            };
            let input = &arg.input;
            let input = match input {
                Some(i) => { Some(i.clone()) },
                None =>  { None }
            };

            let conf = stringer::config::StringerConfig::from(arg);
            match input {
                Some(i) => {
                        stringer_write(i,  conf, output);
                },
                None  => {}
            };
        },
        Err(_) => {
            panic!("error parsing arguments");
        }
    };

}
