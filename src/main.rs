// it's a big mistake
//
use sqlx::sqlite::SqlitePool;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

mod cli;
mod flag;

use flag::Flags;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let matches = cli::create_app().get_matches();

    let flags = Flags::from_matchs(&matches);

    let url = "sqlite:///usr/share/wordlists//stardict.db";

    // let pool = SqlitePool::connect(url).await?;
    if let Ok(pool) = SqlitePool::connect(url).await {
        if flags.interactive {
            let _ = interactive(&pool, &flags).await?;
        } else {
            let default = "abandon".to_string();
            let word = matches.get_one::<String>("WORD").unwrap_or(&default);
            word_query(&pool, word.to_string(), &flags).await?;
        }
    } else {
        println!(
            "Connect to the sqlitedb error, Please confirm this file location: {}",
            url
        );
    }

    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
struct Word {
    word: String,
    translation: String,
    count: i64,
}

async fn interactive(pool: &SqlitePool, flag: &Flags) -> anyhow::Result<()> {
    let mut processing = true;
    println!("Running in interactive mode.Please input words to search.")
    while processing {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                let word = input.trim().to_string();
                let _ = word_query(&pool, word, flag).await?;
            }
            Err(_) => {
                continue;
            }
        }
    }

    Ok(())
}

async fn word_query(pool: &SqlitePool, word: String, flag: &Flags) -> anyhow::Result<Option<Word>> {
    let mut rows = sqlx::query_as::<_, Word>(
        "
        SELECT word,translation, COUNT(*) as count FROM stardict WHERE word = ?
        ",
    )
    .bind(&word)
    .fetch_all(pool)
    .await?;

    while let Some(row) = rows.pop() {
        if row.count == 0 {
            println!("Unable to find the word in the dicitionary: \"{}\". ", word);
            break;
        }
        println!("Word: {}", row.word);
        println!("Translation: \n {}", row.translation);

        if flag.save {
            let _ = write_history(&row);
        }
        return Ok(Some(row));
    }
    Ok(None)
}

fn write_history(words: &Word) -> anyhow::Result<()> {
    let exec_path = env::current_exe().unwrap();
    let p = exec_path.parent().unwrap().join("words.log");

    if !p.exists() {
        std::fs::File::create(&p).unwrap();
    }
    let mut file = OpenOptions::new()
        .append(true)
        .open(p)
        .expect("cannot open log file.");

    file.write_all(words.translation.replace("\n", "\\n").as_bytes())
        .expect("write failed");
    file.write_all("\n".as_bytes());
    Ok(())
}
