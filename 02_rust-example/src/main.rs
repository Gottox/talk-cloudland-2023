use std::fmt;

#[derive(Eq, PartialEq)]
struct Actor {
    name: String,
    year_of_birth: u32,
}

impl Actor {
    fn new(name: &str, year_of_birth: u32) -> Self {
        Self {
            name: name.to_string(),
            year_of_birth,
        }
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.year_of_birth)
    }
}

fn main() {
    println!("Incomplete Movie Database (IMDB)");
    println!("-------------------");

    let mut actors = Vec::new();

    actors.push(Actor::new("Tilda Swinton", 1960));
    actors.push(Actor::new("Tom Hanks", 1956));
    actors.push(Actor::new("Helena Bonham Carter", 1966));
    actors.push(Actor::new("Sandra Bullock", 1964));
    actors.push(Actor::new("Leonardo DiCaprio", 1974));
    actors.push(Actor::new("Tom Hiddleston", 1981));
    actors.push(Actor::new("Scarlett Johansson", 1984));
    actors.push(Actor::new("Liam Neeson", 1952));
    actors.push(Actor::new("Johnny Depp", 1963));

    let tilda_swinton = &actors[0];
    for actor in &actors {
        println!("{}", actor);
    }
    println!("-------------------");
    println!("First actor: {}", tilda_swinton);
    println!("-------------------");
    println!("... Sorting ...");
    actors.sort_by(|a, b| a.year_of_birth.cmp(&b.year_of_birth));
    println!("-------------------");

    for actor in &actors {
        println!("{}", actor);
    }
}
