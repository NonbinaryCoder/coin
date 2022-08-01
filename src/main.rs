mod flip;

fn main() {
    let args = std::env::args();
    let is_tty = atty::is(atty::Stream::Stdout);
    match args.len() {
        1 => flip::flip_once(is_tty),
        _ => {
            let args = args.skip(1).map(|arg| arg.parse()).collect::<Vec<_>>();
            match args.iter().any(|arg| arg.is_err()) {
                true => eprintln!("Args to coin must be integers"),
                false => {
                    let mut gen = rand::thread_rng();
                    for arg in args.into_iter().map(|num| num.unwrap()) {
                        flip::flip_many(is_tty, arg, &mut gen);
                        println!();
                    }
                }
            }
        }
    }
}
