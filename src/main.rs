// TODO: remove allows
#[allow(unused)]
use std::fs::File;
#[allow(unused)]
use std::io::Write;
#[allow(unused)]
use std::path::Path;

mod data;
mod math;
mod parser;

const CONFIG: &'static str = ".ptest.yaml";

fn main() -> std::io::Result<()> {
    let config = crate::data::Config::load(CONFIG);

    // println!("{config:#?}");

    let set = parser::parse();

    // println!("{set:#?}");

    let powerset = math::generate_powerset_combined(&set, config.depth.unwrap());

    // println!("{powerset:#?}");

    // let output_strings: Vec<(Option<String>, Option<String>)> = powerset.iter().collect();
    // .map(|subset| {
    //     format!(
    //         "bin.name = \"{}\"\nargs = \"{} {}\"",
    //         &config.binary.as_ref().unwrap(),
    //         &config.args.as_ref().unwrap(),
    //         subset.join(" ")
    //     )
    // })

    // Go through all elements in the powerset
    // println!("{:#?}", powerset);
    for option_pair in powerset {
        for option in &option_pair {
            let stuff = config.commands.as_ref().expect("NOT_FOUND").get(&option);
            // println!("{thing:?}");
            // println!("{stuff:#?}");
            println!("{:#?}", option);
        }
        //println!("{:#?}", option_pair);
    }

    println!(
        "{:?}",
        config
            .commands
            .as_ref()
            .unwrap()
            .get_key_value(&(Some("-s".to_string()), Some("--sort".to_string())))
    );

    // println!("{config:#?}");

    // Get the Command associated with the key (None, Some("time-style"))
    // if let Some(command) = config
    //     .commands
    //     .as_ref()
    //     .unwrap()
    //     .get(&(None, Some("--time-style".to_string())))
    // {
    //     println!("{:?}", command);
    // } else {
    //     println!("Key not found");
    // }

    // // Get the Command associated with the key (None, Some("time-style"))
    // if let Some(command) = config
    //     .commands
    //     .unwrap()
    //     .get(&(Some("-s".to_string()), Some("--sort".to_string())))
    // {
    //     println!("{:?}", command);
    // } else {
    //     println!("Key not found");
    // }

    // println!("{config:#?}");

    /*
     *
    println!("{output_strings:#?}");

    // println!("Output Strings: {:#?}", output_strings);


    // Create the dump directory if it doesn't exist
    let dump_path = Path::new("dump");
    if !dump_path.exists() {
        fs::create_dir(dump_path).expect("Failed to create dump directory");
    }

    // Write each string in output_strings to a new file in the dump directory
    for content in &output_strings {
        // Extract the arguments part from the content
        let args_line = content.lines().nth(1).unwrap_or("");
        let args = args_line
            .split("args = \"")
            .nth(1)
            .unwrap_or("")
            .trim_end_matches('\"')
            .replace(" ", "_")
            .replace("/", "_"); // This is to handle the "tests/itest" in ARGS

        let file_path = dump_path.join(format!("ptest_{}.toml", args));
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }*/

    Ok(())
}
