use tokio;

mod clip;

use webbase::WebError;

#[tokio::main]
async fn main() -> Result<(), WebError> {
    let args = clip::parse();
    let body = if args.is_present("body") {
		String::from(
			args.value_of("body")
				.expect("Problem on read body argument."),
		)
	} else {
		let body_err_msg = "You let me as an errant soul. You must give me a body, as an argument -b / --body or with the environment variable WEBBASE_BODY";
		let body_env = std::env::var_os("WEBBASE_BODY").expect(body_err_msg);
		let body_env = body_env.to_str().expect(body_err_msg);
		format!("{}", body_env)
	};
    if let Some(input) = args.value_of("input") {
		webbase::make(input, &body).await?;
	}
    Ok(())
}
