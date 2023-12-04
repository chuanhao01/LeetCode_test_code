use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d2")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for l in input.split("\r\n") {
        let l = l.split(": ").collect::<Vec<&str>>();
        let id = l.first().unwrap();
        let games = l.last().unwrap();
        let id = id.replace("Game ", "");
        let id = id.parse::<i64>().unwrap();
        // For each game
        let options = games.split("; ").fold(
            HashMap::from([("red", 0), ("blue", 0), ("green", 0)]),
            |mut acc: HashMap<&str, i64>, game| {
                for option in game.split(", ") {
                    let option = option.split(' ').collect::<Vec<&str>>();
                    // println!("{:?}", option);
                    let v = option.first().unwrap().parse::<i64>().unwrap();
                    let k = option.last().unwrap();
                    acc.insert(k, *acc.get(k).unwrap().max(&v));
                }
                acc
            },
        );
        sum += if options.get("red") <= possible.get("red")
            && options.get("green") <= possible.get("green")
            && options.get("blue") <= possible.get("blue")
        {
          id
        } else {0};
        println!("{:?}", options);
        println!("{}", id);
    }

    println!("{:?}", sum);
    Ok(())
}
