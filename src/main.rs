use ini::Ini;
use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("Name"))]
    select: String,

    #[arg(short, long, num_args(0..))]
    appdir: Vec<String>,

    #[arg(short, long, num_args(0..))]
    filter: Vec<String>,
}

fn parse_desktop_file(path: &str) -> Result<HashMap<String, String>, &'static str> {
    let i = Ini::load_from_file(path).unwrap();
    let entry = i.section(Some("Desktop Entry"));

    if !entry.is_some() {
        return Err("No desktop entry");
    }


    match entry {
        Some(e) => {
            let mut map: HashMap<String, String> = HashMap::new();

            for (k, v) in e.iter() {
                map.insert(k.to_string().to_lowercase(), v.to_string());
            }

            Ok(map)
        },
        None => Err("No Desktop Entry section")
    }
}

fn append_app_dir<'a>(v: &mut Vec<&'a str>, path: &'a str) {
    let meta = fs::metadata(path);

    if meta.is_err() {
        return ;
    }

    if !meta.unwrap().is_dir() {
        return ;
    }

    v.push(path);
}

fn make_local_app_dir_path() -> String {
    let home = env::var("HOME").unwrap_or("/root".to_string());

    format!("{}/.local/share/applications", home)
}

fn list_desktop_entries(dirs: Vec<&str>) -> Vec<HashMap<String, String>> {
    let mut v: Vec<HashMap<String, String>> = Vec::new();
    let mut app_names: HashSet<String> = HashSet::new();

    for dir in dirs {
        let rdir = fs::read_dir(dir);

        if rdir.is_err() {
            continue ;
        }

        for entry in rdir.unwrap() {
            if entry.is_err() {
                continue ;
            }

            let e = entry.unwrap();

            if e.metadata().unwrap().is_dir() {
                continue ;
            }

            let path = e.path();
            let spath = path.to_str().unwrap();

            if !spath.ends_with(".desktop") {
                continue ;
            }

            let desktop = parse_desktop_file(spath);
            
            if desktop.is_err() {
                continue ;
            }

             let name: &String = desktop.as_ref().unwrap().get("name").unwrap();

             if app_names.contains(name) {
                 continue; 
             }

             app_names.insert(name.to_string());
             v.push(desktop.unwrap());
        }
    }

    return v;
}

fn main() {
    let cli = Cli::parse();
    let mut desktop_file_dirs: Vec<&str> = Vec::new();
    let local_app_dir = make_local_app_dir_path();

    append_app_dir(&mut desktop_file_dirs, "/usr/share/applications");
    append_app_dir(&mut desktop_file_dirs, "/usr/local/share/applications");
    append_app_dir(&mut desktop_file_dirs, &local_app_dir);

    for appdir in &cli.appdir {
        append_app_dir(&mut desktop_file_dirs, appdir)
    }

    let entries = list_desktop_entries(desktop_file_dirs);
    let mut parsed_filters: Vec<(String, Regex)> = Vec::new();

    for filter in &cli.filter {
        let parts: Vec<&str> = filter.split(",").collect();

        if parts.len() != 2 {
            eprintln!("Ill-formated filter: {}", filter);

            return 
        }

        let rex = Regex::new(parts[1]);

        if rex.is_err() {
            eprintln!("Invalid regular expression: {}", parts[1]);

            return
        }

        parsed_filters.push((parts[0].to_lowercase(), rex.unwrap()));
    }
    
    for entry in entries {
        let mut is_filtered = true;

        for (k, rex) in &parsed_filters {
            let v = entry.get(k);
            
            if v.is_none() {
                is_filtered = false;
                break ;
            }

            is_filtered = rex.is_match(v.unwrap());
        }

        if !is_filtered {
            continue ;
        }

        match entry.get(&cli.select.to_lowercase()) {
            Some(v) => {
                println!("{}", v);
            },
            None => {}
        }
    }
}
