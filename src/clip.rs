use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
	App::new("webbase")
		.version(clap::crate_version!())
		.author("Éverton M. Vieira <everton.muvi@gmail.com>")
		.about("WebBase - Generates database indexes from websites for fast content search.")
		.arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .value_name("DIR")
                .takes_value(true)
                .required(false)
                .help("To where the databases indexes will be stored.")
        )
		.arg(
			Arg::with_name("input")
				.short("i")
				.long("input")
				.value_name("URL")
				.takes_value(true)
				.required(true)
				.help("From where the content will be downloaded.")
		)
		.get_matches()
}
