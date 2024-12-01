pub fn input_file(caller: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        args[1].clone()
    } else {
        format!(
            "input/{}",
            caller.to_string()[caller.len() - 5..caller.len() - 3].to_string()
        )
    }
}
