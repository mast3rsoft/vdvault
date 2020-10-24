use strsim::levenshtein;
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use regex::*;
use std::process::*;
use rustyline::Editor;
use ansi_term::Color;


fn find_dir(to_match: &str, media: &str) -> Option<DirEntry> {
     let media_folder = read_dir(media).expect("media");
    let mut top_path =  PathBuf::new();
    let mut top_levenstein = 1000;
    let mut top_entry = None;
  
    for direntry in media_folder {
        let entry = direntry.expect("Baf dir entry");
        let dir_name = entry.file_name();
        let dir_name = dir_name.to_str().unwrap();
        let regex = format!(".*{}.*",to_match );
        println!("Regex is {}", regex);
        let dir_lebenstein = levenshtein(to_match,  dir_name );
        let mut dir_regex = RegexBuilder::new(regex.as_str());
        dir_regex.ignore_whitespace(true);
        dir_regex.case_insensitive(true);
    
        let dir_regex = dir_regex.build().unwrap();
            
        println!("Lebenstein for {} is {:?}", dir_lebenstein, entry.path() );
        if  dir_regex.is_match(dir_name) {
            println!("Regex");
            top_path = entry.path();
             top_entry = Some(entry);
             return top_entry
        } else if dir_lebenstein < top_levenstein  {
             
             println!("Higher tan before {:#?} ", entry.file_name());
             top_levenstein = dir_lebenstein;
             top_path = entry.path();
             top_entry = Some(entry);
         }

    }
   let entri = &top_entry;
   
   if entri.as_ref().unwrap().file_type().unwrap().is_dir() {
        println!("Found directory {:#?}. Please be more specific... ", entri.as_ref().unwrap().file_name());
        
        let mut more_specific_path = String::new();
        let mut rl = Editor::<()>::new();
        
        let readline = rl.readline(">>");
        match readline {
            Ok(line) => {
                more_specific_path = line
            }, 
            Err(_) => {
                println!("Bye");
                std::process::exit(1);
                
            }
        }

         
        let mut specicficPathByf = PathBuf::new();
        specicficPathByf.push(entri.as_ref().unwrap().path());
       println!("{:?}", specicficPathByf);
        return find_dir(&more_specific_path,specicficPathByf.to_str().unwrap());
   }
   
   top_entry
    
}
fn main() {
    
    let  cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() == 1 {
       println!("{}", Color::Red.bold().paint("Not enough args"));
       std::process::exit(1);
   }
    let  cli_args = cli_args.as_slice();
    let  cli_args = &cli_args[1..];
    let  cli_args = Vec::from(cli_args);
    let  cli_args =  cli_args.join(" ");

    println!("Hey thr args {:?}", cli_args);


   let  entry = &find_dir(cli_args.as_str(), "/Volumes/media/Videos/");
   println!("The best match is {:#?}", entry.as_ref().unwrap().file_name());
   println!("Opening...");
   let entry_path = entry.as_ref().unwrap().path();
   let entry_path_str = entry_path.as_os_str().to_str();
   
   std::process::Command::new("open")
        .arg("-a")
        .arg("VLC")
        .arg(entry_path_str.unwrap())
        .spawn()
        .expect("Failure launching vlc")
        ;

        

   
   

}
