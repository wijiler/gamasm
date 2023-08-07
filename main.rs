use std::{env, process::{exit,Command}};
fn main () {
     let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("ARGS not supplied");
        exit(1);
    }
    match args[1].as_str() {
        "-f" | "-file" => {
            let oargs:&String = &args[2].to_string().replace(".s",".o");
            let ldoargs = &args[2].to_string().replace(".s","");
            Command::new("as")
                .arg("-o")
                .arg(oargs.to_string())
                .arg(args[2].to_string())
                .spawn()
                .expect("oof it dont work");
           Command::new("ld")
               .arg(oargs.to_string())
               .arg("-o")
               .arg(ldoargs)
               .spawn()
               .expect("oop it no work check your code");
        }
        _ => { println!("use -f or --file to compile your gas"); exit(1);}
    }
}
