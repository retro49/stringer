extern crate stringer;

fn stringer_file(path: String, conf: stringer::config::StringerConfig, out: Option<String>) {
    let file = std::fs::File::open(path);
    match file {
        Ok(f) => {
            let mut file = f;
            let strgr = 
                stringer::Stringer::new(&mut file);
            match strgr {
                Ok(s) => {
                    let mut s = s;
                    s.set_config(conf);
                    s.read_strings();
                    let res = s.results;
                    for r in res {
                        let sr = serde_json::to_string(&r).unwrap();
                        println!("{}", sr);
                    }
                },
                Err(_) => {
                    println!("unable to extract strings");
                }
            };
        },
        Err(_) => {
            println!("unable to open file")
        },
    };
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
                        stringer_file(i,  conf, output);
                },
                None  => {}
            };
        },
        Err(_) => {
            panic!("error parsing arguments");
        }
    };

}
