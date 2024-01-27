use std::env;
use std::{collections::HashMap, fs, process::Command, str, time::Instant};
use rayon::prelude::*;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Query {
    name: String,
    sql: String,
}

fn main() {
    
    let db_file: String = env::args().nth(1).unwrap();   
    let tomlfile: String = env::args().nth(2).unwrap();      
    let outpath: String = env::args().nth(3).unwrap();
    
    let now = Instant::now();
    
    let content = fs::read_to_string(tomlfile).unwrap();
    let queries_table: HashMap<String, Vec<Query>> = toml::from_str(&content).unwrap();
    let queries: &Vec<Query> = queries_table.get("query").unwrap();

    //let sqlite_binary = "C:\\Users\\XXXX\\.pyenv\\pyenv-win\\versions\\3.8.8\\sqlite3.exe";
    let qrun: usize = queries
        .par_iter()
        .map(|q| {
            let name = q.name.to_string();
            let sql = q.sql.to_string();
            let outfile = format!("{}/{}.json", outpath.replace('\\', "/"), name);            
            
            let _output = Command::new("sqlite3")                
                .arg(&format!("{}", db_file))   
                .args(&[".load mod_spatialite"])
                .args(&[".mode json"])     
                .args(&[format!(".once {}", outfile)]) 
                .arg(&format!("{}", sql))                      
                .output()
                .expect("failed to execute process");

            1 // return 1 to indicate query execution
        })
        .sum();
    let elapsed: std::time::Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("{:?}", qrun);
}
