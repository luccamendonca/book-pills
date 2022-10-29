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

fn parse_book(file_path: &str, desired_page: usize) -> Vec<String> {
    let doc = EpubDoc::new(file_path);
    assert!(doc.is_ok());

    let mut doc = doc.unwrap();
    let title = doc.mdata("title");
    print!("\n");
    println!("The book title is: {}", title.unwrap());
    println!("The book is {} pages long.", doc.resources.len());

    let mut response_text: Vec<String> = vec![];

    match doc.set_current_page(desired_page) {
        Ok(_) => (),
        Err(e) => {
            response_text.push(format!(
                "error setting current page to {desired_page:?}: {e:?}"
            ));
            return response_text;
        }
    };

    let page: String = doc.get_current_str().unwrap();
    let page_string_len: usize = page.len();
    let page_content: String = from_read(page.as_bytes(), page_string_len);
    let page_content_bytes_len: usize = page_content.as_bytes().len();

    let step_size: usize = 4096;
    let mut l: usize = 0;
    let mut r: usize;
    if step_size > page_content_bytes_len {
        r = page_content_bytes_len - 1
    } else {
        r = step_size
    }

    while page_content_bytes_len >= r {
        response_text.push(page_content[l..r].to_string());
        l = r;
        if r + step_size > page_content_bytes_len && r < page_content_bytes_len {
            r += page_content_bytes_len - r;
            continue;
        }
        r += step_size;
    }

    return response_text;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::NewBook => {
            let split_book_page = parse_book("gone_girl.epub", 9);
            // println!("----------------\nMessage 1: {}", split_book_page[0]);
            // println!("----------------\nMessage 2: {}", split_book_page[1]);

            bot.send_message(msg.chat.id, split_book_page[0].as_str())
                .await?;
            bot.send_message(msg.chat.id, split_book_page[1].as_str())
                .await?
        }
    };

    Ok(())
}
