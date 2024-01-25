use clap::Parser;
use find_and_replace::*;


fn main(){
    let args = Args::parse();
    let data = open_file(&args.input);

    let result = replace(&data, &args.find, &args.replace);
    save_to_file(&args.output, &result);
}
