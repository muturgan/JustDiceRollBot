use ::std::env::var as readEnvVar;
use futures::stream::StreamExt;
use rand::Rng;
use telegram_bot::{Api, CanReplySendMessage, Error as TelegramError, MessageKind, UpdateKind};

#[tokio::main]
async fn main() -> Result<(), TelegramError> {
	let token = readEnvVar("BOT_TOKEN").expect("BOT_TOKEN environment variable is not defined");
	let _bot_name = readEnvVar("BOT_NAME").expect("BOT_NAME environment variable is not defined");

	let api = Api::new(token);

	let mut stream = api.stream();
	while let Some(update) = stream.next().await {
		// If the received update contains a new message...
		let update = update?;
		if let UpdateKind::Message(message) = update.kind {
			if let MessageKind::Text { ref data, .. } = message.kind {
				// Print received text message to stdout.
				println!("<{}>: {}", &message.from.first_name, data);

				// Answer message with "Hi".
				api.send(message.text_reply(format!(
					"Hi, {}! You just wrote '{}'",
					&message.from.first_name, data
				)))
				.await?;
			}
		}
	}
	Ok(())
}

pub fn roll(msg: &str) -> String {
	let splited = msg
		.to_lowercase()
		.trim()
		.split("d")
		.map(|s| s.to_string().parse::<u8>().unwrap_or(1))
		.collect::<Vec<u8>>();

	let dice_count = splited[0];
	let faces_count = splited[1];

	let mut results: Vec<u8> = vec![];

	for _ in 1..dice_count {
		results.push(rand::thread_rng().gen_range(1..faces_count));
	}

	if results.len() == 1 {
		return results[0].to_string();
	}

	let mut sum: u8 = 0;

	let expr = results
		.iter()
		.map(|n| {
			sum += n;
			n.to_string()
		})
		.collect::<Vec<String>>()
		.join(" + ");

	return format!("{expr} = {sum}");
}
