use epub::doc::EpubDoc;
use html2text::from_read;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Testing commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle book file")]
    NewBook,
}

// async fn parse_book(epub_file_path: String, desired_page: usize) -> String {
fn parse_book() -> Vec<String> {
    let epub_file_path = String::from("gone_girl.epub");
    let desired_page = 10;

    let doc = EpubDoc::new(epub_file_path);
    assert!(doc.is_ok());

    let mut doc = doc.unwrap();
    let title = doc.mdata("title");
    print!("\n");
    println!("The book title is: {}", title.unwrap());
    println!("The book is {} pages long.", doc.resources.len());

    let mut response_text: Vec<String> = vec![];

    match doc.set_current_page(desired_page) {
        Ok(_) => (),
        Err(e) => response_text.push(format!(
            "error setting current page to {desired_page:?}: {e:?}"
        )),
    };

    match doc.get_current_str() {
        // Ok(v) => response_text.append(&mut vec![format!("\n{}", from_read(v.as_bytes(), v.chars().count()))]),
        Ok(page) => {
            let page_content = from_read(page.as_bytes(), page.chars().count());
            println!("string size: {}", page.chars().count());
            response_text.push(page_content[..1024].to_string());
        }
        Err(e) => response_text.push(format!("error getting page content: {e:?}")),
    };

    return response_text;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::NewBook => {
            let mut msg_text = String::from("");
            for book_page in parse_book() {
                msg_text = book_page;
            }
            bot.send_message(msg.chat.id, msg_text).await?
        }
    };

    Ok(())
}
