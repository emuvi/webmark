use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
	App::new("webmark")
		.version(clap::crate_version!())
		.author("Éverton M. Vieira <everton.muvi@gmail.com>")
		.about("WebMark - Downloads the contents, texts and images, of a single web address and stores as a markdown file.")
		.arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .value_name("DIR")
                .takes_value(true)
                .required(false)
                .help("To where the contents will be stored.")
        )
		.arg(
			Arg::with_name("input")
				.short("i")
				.long("input")
				.value_name("URL")
				.takes_value(true)
				.required(true)
				.help("From where the contents will be downloaded.")
		)
		.get_matches()
}
