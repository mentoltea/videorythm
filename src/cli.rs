use crate::logic::Arguments;
use std::path::Path;


pub fn parse_command_line (args: &[String]) -> Result<Arguments, String> {
    let mut work_dir: Option<String> = None;
    let mut original_dir: Option<String> = None;  
    let mut edited_dir: Option<String> = None;  
    let mut infile: Option<String> = None;  
    let mut outfile: Option<String> = None;  
    let mut sciptfile: Option<String> = None;  
    let mut use_cached: Option<bool> = None;  
    let mut fps: Option<u16> = None;  
    let mut threads: Option<u16> = None;  
    

    let pairs: Vec< ( &String, &String ) > = args
        .chunks_exact(2)
        .map(|chunk| { (&chunk[0], &chunk[1]) })
        .collect();

    for (lhs, rhs) in pairs {
        match &lhs[0..] {
            "--working" => { work_dir = Some(rhs.to_owned()) },

            "--original_dir" => { original_dir = Some(rhs.to_owned()) },
            "--edited_dir" => { edited_dir = Some(rhs.to_owned())},
            
            "--input" | "-i" => { infile = Some(rhs.to_owned())},
            "--output" | "-o" => { outfile = Some(rhs.to_owned())},
            "--script" | "-s" => { sciptfile = Some(rhs.to_owned())},

            "--threads" | "-t" => {
                let num: u16 = rhs.parse().unwrap();
                threads = Some(num);
            },

            "--fps" => {
                let num: u16 = rhs.parse().unwrap();
                fps = Some(num);
            },
            "--use_cached" => {
                match &rhs[0..] {
                    "true" | "True" | "1" => { use_cached = Some(true)} ,
                    "false" | "False" | "0" => { use_cached = Some(false)} ,
                    _ => { return Err(format!("Cannot interpret {rhs} as boolean")); } ,
                }
            },
            _ => { return Err(format!("Unknown argument {lhs}")) }, 
        }
    }
    
    if infile.is_none() {
        return Err(format!("Input file by --input is expected"));
    } 
    let input_path = Path::new(&infile.clone().unwrap()).to_owned();
    let input_filename = input_path.file_stem().unwrap().to_str().unwrap();
    let input_extension = input_path.extension().unwrap().to_str().unwrap();

    if sciptfile.is_none() {
        return Err(format!("Lua script file by --script is expected"));
    } 

    let mut result = Arguments {
        work_dir: work_dir.unwrap_or(format!("./{input_filename}_pwd/")),

        original_dir: original_dir.unwrap_or(format!("original_frames/")),
        edited_dir: edited_dir.unwrap_or(format!("edited_frames/")),
        
        input_file: infile.unwrap(),
        output_file: outfile.unwrap_or(format!("./{input_filename}_out.{input_extension}")),
        
        script_file: sciptfile.unwrap(),
        
        use_cached: use_cached.unwrap_or(true),
        fps: fps.unwrap_or(30),

        threads: threads.unwrap_or(0),
    };

    if !result.work_dir.ends_with("/") { result.work_dir.push('/');}
    if !result.original_dir.ends_with("/") { result.original_dir.push('/');}
    if !result.edited_dir.ends_with("/") { result.edited_dir.push('/');}

    return Ok(result);
}