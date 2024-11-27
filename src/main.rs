use nix::dir::Dir;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;

fn list(path: &str) {
    let directory = Dir::open(path, OFlag::O_DIRECTORY, Mode::S_IXUSR).expect("Fail to open");

    let ls = directory
        .into_iter()
        .map(|x| x.unwrap().file_name().to_string_lossy().to_string());

    let result = ls.reduce(|a, b| a + "\t" + &b);
    println!("{}", result.unwrap());
}

fn main() {
    list("./");
}
