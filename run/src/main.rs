mod run;

fn main() {
    run::day01();
    run::day02();
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     p: asref<path>,
// {
//    let file = file::open(filename)?;
//    ok(io::bufreader::new(file).lines())
// }
