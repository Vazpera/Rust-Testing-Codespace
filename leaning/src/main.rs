enum OptOrRes<T> {
    Opt(Option<T>),
    Res(Result<T, ()>),
}

fn convert<T>(input: OptOrRes<T>) -> OptOrRes<T> {
    match input {
        OptOrRes::Opt(j) => match j {
            Some(k) => OptOrRes::Res(Ok(k)),
            None => OptOrRes::Res(Err(())),
        },
        OptOrRes::Res(j) => match j {
            Ok(k) => OptOrRes::Opt(Some(k)),
            Err(_) => OptOrRes::Opt(None),
        },
    }
}

fn main() {
    println!("Hello, world!");
}
