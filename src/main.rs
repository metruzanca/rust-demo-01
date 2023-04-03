use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines, Write};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use chrono;

// TODO kill unwraps

#[derive(Deserialize, Serialize, Debug)]
struct Message {
  timestamp: i64,
  r#type: String,
  sender: String,
  message: String,
}

fn read_file(path: &str) -> Lines<BufReader<File>>  {
  let file = File::open(path).unwrap();
  let lines = BufReader::new(file).lines();
  return lines
}

fn write_file(path: &str, data: String) {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();

  if let Err(e) = writeln!(file, "{}", data) {
    eprintln!("Couldn't write to file: {}", e);
  }
}

fn logs_to_messages(lines: Lines<BufReader<File>>) -> Vec<Message> {  
  let mut messages: Vec<Message> = Vec::new();
  
  for line in lines {
    let raw_line = line.unwrap();
    let message = serde_json::from_str::<Message>(&raw_line).unwrap();
    messages.push(message);
  }

  return messages;
}

fn format_message(message: Message) -> String {
  let sender = format!("{:width$}", message.sender, width=10);
  let message = format!("{}: {}", sender, message.message);
  return message;
}

fn help() {
  println!("Usage: rustord <command> [args]");
  println!("Commands:");
  println!("  send <message> - Send a message");
  println!("  read           - Read messages");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  
  match args.len() {
    // This is a called a match guard
    l if l > 1 => {
      let command = &args.get(1).ok_or("default").unwrap();
      match command.as_str() {
        "read" => {
          let logs = read_file("messages.log");
          let messages = logs_to_messages(logs);
          for message in messages {
            println!("{}", format_message(message));
          }
        },
        "send" => {
          let message = args[2..].join(" ");
          // Write a message to the log file
          let timestamp = chrono::Utc::now().timestamp();
          let message = Message {
            timestamp,
            r#type: "message".to_string(),
            sender: "Sam".to_string(),
            message,
          };

          let json = serde_json::to_string(&message).unwrap();
          write_file("messages.log", json);
        },
        _ => {}
      }
    }
    _ => {
      help();
      return;
    }
  }

}
