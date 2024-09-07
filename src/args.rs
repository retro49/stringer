/// Default arguments that are accepted by stringer
#[derive(Default)]
pub struct Args {
    /// window mimum search size option
    pub window_min_size: Option<u64>,
    /// window maximum size
    pub window_max_size: Option<u64>,
    /// special characters inclusing option
    pub special: Option<bool>,
    /// whitespace inclusion option
    pub whitespace: Option<bool>,
    /// output fomat option
    pub output_format: Option<String>,
    /// length option to be included in ouput
    pub length: Option<bool>,
    /// regex option
    pub regex: Option<String>,
    /// output file
    pub output: Option<String>,
    /// input file or . for directory
    pub input: Option<String>,
}

impl Args {
    // just parses from args, using clap.
    fn parse_cmd() -> clap::ArgMatches {
        let cmd = clap::Command::new("stringer")
            .arg(
                clap::Arg::new("min")
                    .short('m')
                    .long("min")
                    .help("minimum window size for searching, minimum is 4")
                    .required(false)
                    .default_value("4")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                clap::Arg::new("max")
                    .short('M')
                    .long("max")
                    .help("maximum window size for searching, 0 means not limited by size")
                    .required(false)
                    .default_value("0")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                clap::Arg::new("special")
                    .short('s')
                    .long("special")
                    .help("allows special characters to be included")
                    .default_value("false")
                    .required(false)
                    .value_parser(clap::value_parser!(bool)),
            )
            .arg(
                clap::Arg::new("whitespace")
                    .short('w')
                    .long("whitespace")
                    .help("allows whitespaces to be included")
                    .default_value("false")
                    .required(false)
                    .value_parser(clap::value_parser!(bool)),
            )
            .arg(
                clap::Arg::new("format")
                    .short('f')
                    .long("format")
                    .help("output format of the strings. JSON, XML, YAML, LITERAL")
                    .default_value("literal")
                    .required(false)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("length")
                    .short('l')
                    .long("length")
                    .help("write the length of the string on the output")
                    .default_value("false")
                    .required(false)
                    .value_parser(clap::value_parser!(bool)),
            )
            .arg(
                clap::Arg::new("out")
                    .short('o')
                    .long("out")
                    .help("output file")
                    .required(false)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("in")
                    .short('i')
                    .long("in")
                    .help("input file to extract")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("regex")
                    .short('r')
                    .long("regex")
                    .help("pattern for searching specific strings")
                    .required(false)
                    .value_parser(clap::value_parser!(String)),
            )
            .color(clap::ColorChoice::Auto);
        cmd.get_matches()
    }

    /// returns a parsed arguments from arguments
    pub fn parse_args() -> Result<Args, crate::error::StringerError> {
        let cmd = Self::parse_cmd();
        Ok(Args {
            window_min_size: cmd.get_one::<u64>("min").copied(),
            window_max_size: cmd.get_one::<u64>("max").copied(),
            special: cmd.get_one::<bool>("special").copied(),
            whitespace: cmd.get_one::<bool>("whitespace").copied(),
            output_format: match cmd.get_one::<String>("format") {
                Some(f) => Some(f.clone()),
                _ => None,
            },
            length: cmd.get_one::<bool>("length").copied(),
            output: match cmd.get_one::<String>("out") {
                Some(s) => Some(s.clone()),
                _ => None,
            },
            input: match cmd.get_one::<String>("in") {
                Some(s) => Some(s.clone()),
                _ => None,
            },
            regex: match cmd.get_one::<String>("regex") {
                Some(s) => Some(s.clone()),
                _ => None,
            },
        })
    }
}
