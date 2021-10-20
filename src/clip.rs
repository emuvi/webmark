use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
	App::new("webbase")
		.version(clap::crate_version!())
		.author("Éverton M. Vieira <everton.muvi@gmail.com>")
		.about("WebBase - Constructs databases from websites.")
		.arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .value_name("DIR")
                .takes_value(true)
                .required(true)
                .help("Where all databases are constructed.")
        )
		.arg(
			Arg::with_name("speed")
				.short("e")
				.long("speed")
				.value_name("NUMBER")
				.default_value("4")
				.takes_value(true)
				.required(false)
				.help("How fast should I go.")
		)
		.arg(
			Arg::with_name("input")
				.short("i")
				.long("input")
				.value_name("URL")
				.takes_value(true)
				.required(false)
				.help("From where should I get the content?"),
		)
		.arg(
			Arg::with_name("depth-up")
				.short("u")
				.long("depth-up")
				.value_name("NUMBER")
				.default_value("0")
				.takes_value(true)
				.required(false)
				.help("Should I go up on the website hierarch?"),
		)
		.arg(
			Arg::with_name("depth-down")
				.short("d")
				.long("depth-down")
				.value_name("NUMBER")
				.default_value("0")
				.takes_value(true)
				.required(false)
				.help("Should I go down on the website hierarch?"),
		)
		.arg(
			Arg::with_name("depth-out")
				.short("o")
				.long("depth-out")
				.value_name("NUMBER")
				.default_value("0")
				.takes_value(true)
				.required(false)
				.help("Should I go down on the website hierarch?"),
		)
		.arg(
			Arg::with_name("search")
				.short("s")
				.long("search")
				.value_name("WORDS")
				.takes_value(true)
				.required(false)
				.help("Searches in the databases.")
		)
		.get_matches()
}
