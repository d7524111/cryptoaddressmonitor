use core::panic;
use std::{thread::{self, current}, time};
use clipboard_rs::{Clipboard, ClipboardContext, ContentFormat};
use colored::Colorize;
use regex::Regex;

fn main() {

    let btc_re = Regex::new(r"^(bc1|[13])[a-zA-HJ-NP-Z0-9]{25,39}$").unwrap();
    let eth_re = Regex::new(r"0x[a-fA-F0-9]{40}").unwrap();
    let xmr_re = Regex::new(r"^[48][1-9A-HJ-NP-Za-km-z]{94}$").unwrap();



    println!("{}", "watching clipboard".green().bold());

    let ctx = ClipboardContext::new().unwrap();
    let mut last_content = ctx.get_text().unwrap_or_default();
    println!("Current clipboard: ");
    println!("{}", last_content.red());
    
    loop{
        let mut current_content = ctx.get_text().unwrap_or_default();
        
        match current_content.as_str(){
            c if btc_re.is_match(c) => {
                println!("Btc address detected: {}", c);
            }
            c if eth_re.is_match(c) => {
                println!("eth detected: {}", c);
            }
            c if xmr_re.is_match(c) => {
                println!("xmr detected: {}", c);
            }
            _ => {
                println!("no address detected..");
            }
        }
        thread::sleep(time::Duration::from_millis(10000));
    }
}


