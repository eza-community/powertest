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
    println!("{:#?}", powerset);
    for options in powerset {
        let mut res_options: Vec<(Option<String>, Option<String>)> = vec![];

        for option in options {
            //if let Some(stuff) = config.commands.as_ref().unwrap().get(&option) {
            if let Some((key, stuff)) = config.commands.as_ref().unwrap().get_key_value(&option) {
                match key {
                    (Some(left), Some(right)) => {
                        for value in stuff.values.as_ref().unwrap() {
                            println!("{} {}", left, &value);
                            println!("{} {}", right, &value);
                        }
                    }
                    (Some(left), None) => {
                        for value in stuff.values.as_ref().unwrap() {
                            println!("{} {}", left, &value);
                        }
                    }
                    (None, Some(right)) => {
                        for value in stuff.values.as_ref().unwrap() {
                            println!("{} {}", right, &value);
                        }
                    }
                    (None, None) => todo!(),
                }
            } else {
                match &option {
                    (Some(left), Some(right)) => {
                        println!("{}", left);
                        println!("{}", right);
                    }
                    (Some(left), None) => {
                        println!("{}", left);
                    }
                    (None, Some(right)) => {
                        println!("{}", right);
                    }
                    (None, None) => todo!(),
                }
                // println!("{option:?}");
                res_options.push(option);
            }
        }
    }

    // This approach doesn't work!
    //
    // Because... It assumes depth, but it can vary!
    // for (left, right) in powerset {
    //     let res: Vec<Option<String>> = vec![];

    //     match (
    //         config.commands.as_ref().expect("NOT_FOUND").get(&left),
    //         config.commands.as_ref().expect("NOT_FOUND").get(&right),
    //     ) {
    //         (Some(left), Some(right)) => println!("left right"),
    //         (Some(left), None) => println!("left right"),
    //         (None, Some(right)) => println!("left right"),
    //         (None, None) => println!("left right"),
    //     }

    //     for option in &option_pair {
    //         if let Some(stuff) = config.commands.as_ref().expect("NOT_FOUND").get(&option) {
    //             println!("{stuff:#?}");
    //         }
    //         // println!("{thing:?}");
    //         // println!("{:#?}", option);
    //     }
    //     // println!("{:#?}", option_pair);
    // }

    // println!(
    //     "{:?}",
    //     config
    //         .commands
    //         .as_ref()
    //         .unwrap()
    //         .get_key_value(&(Some("-s".to_string()), Some("--sort".to_string())))
    // );

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
