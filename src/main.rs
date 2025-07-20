mod errors;
mod command;
mod helpers;

use errors::CrateResult;
use command::Command;

use tokio::{
    io::{AsyncBufReadExt,AsyncWriteExt},
    task::JoinHandle,
};

use crate::helpers::pwd;

fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        // init stdin and stdout
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::BufWriter::new(stdout);
        stdout.write(b"Shell session started\n").await?;
        stdout.write(pwd()?.as_bytes()).await?;
        stdout.write(b"\n>").await?;
        stdout.flush().await?;
        while let Ok(Some(line)) = reader.next_line().await {
            let command = handle_new_line(&line).await;
        
            if let Ok(command) = &command {
                match command {
                    Command::Exit => {
                        println!("Good Bye...");
                        break;
                    }
                    Command::Echo(s) => {
                        println!("{}",s);
                    }
                    _ => {}
                }
            } else {
                eprintln!("Error parsing command: {}", command.err().unwrap());
            }
            stdout.write(pwd()?.as_bytes()).await?;
            stdout.write(b"\n>").await?;
            stdout.flush().await?;
        }

        Ok(())
    })
}
async fn handle_new_line(line:&str) -> CrateResult<Command> {
    let command : Command = line.try_into()?;
    match command.clone() {
        Command::Ls => { helpers::ls()?;},
        Command::Pwd => { println!("{}",helpers::pwd()?);},
        Command::Cd(path) => { helpers::cd(&path)?; },
        Command::Touch(s) => { helpers::touch(&s)?;},
        Command::Rm(s) => { helpers::rm(&s)?;},
        Command::Cat(s) => {
            let contents = helpers::cat(&s)?;
            println!("{}",contents)
        }
        _ => {},
    }
    Ok(command)
}
#[tokio::main]
async fn main() {
    let user_input_handler = spawn_user_input_handler().await;
    if let Ok(Err(e)) = user_input_handler {
        eprintln!("Error: {}", e);
    }
}
