use core::panic;
use std::{thread, time};
use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};
use colored::Colorize;

fn main() {

    println!("{}", "watching clipboard".green().bold());
    let ctx = ClipboardContext::new().unwrap();
    let mut last_content = ctx.get_text().unwrap_or_default();
    println!("Current clipboard: ");
    println!("{}", last_content.red());
    
    loop{
        let current_content = ctx.get_text().unwrap_or_default();
        
        if current_content != last_content{
            println!("content changed! aborting");
            println!("Clipboard changed to : ");
            println!("{}", current_content);
            break;
        } else{
            println!("no change");
        }


        let sleep = time::Duration::from_millis(10000);
        thread::sleep(sleep);
    }
}
