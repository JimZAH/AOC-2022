fn main() {
    let f = std::fs::read_to_string("input.txt").unwrap();
    let mut res = Vec::new();

    let mut n: u32 = 0;
    for i in f.split('\n') {
        if i.is_empty() {
            res.push(n);
            n = 0;
            continue;
        }
        n += i.parse::<u32>().unwrap();
    }

    let mut hr = 0;
    res.sort_unstable();
    for i in &res[res.len() - 3..res.len()] {
        println!("{i}");
        hr += *i;
    }

    println!("Top: {}, Top 3 combined: {hr}", res.last().unwrap());
}
