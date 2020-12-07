use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2020::common::inputs;

struct ContainedBag {
    n: i32,
    attrs: String,
}

struct Bag {
    attrs: String,
    contained_bags: Vec<ContainedBag>,
}

fn main() {
    let bags = inputs::fread_lines("src/bin/day7/input")
        .iter()
        .map(|spec| to_bag(spec))
        .collect::<Vec<Bag>>();

    for bag in bags.iter() {
        println!("Bag: {}", bag.attrs);
        for contained_bag in bag.contained_bags.iter() {
            println!("  {} {}", contained_bag.n, contained_bag.attrs)
        }
    }

    let containers = attr_to_containers(&bags);

    let containers_shiny_gold = containers_of(String::from("shiny gold"), &containers);
    println!(
        "Containers of shiny gold: {:#?} - {}",
        containers_shiny_gold,
        containers_shiny_gold.len()
    );

    let containings = attrs_to_containing(&bags);
    let containings_sg = containings_of("shiny gold", &containings);
    println!("A shiny gold contains {} bags", containings_sg);
}

fn containers_of(
    attrs: String,
    all_containers: &HashMap<String, HashSet<String>>,
) -> HashSet<&str> {
    let mut containers: HashSet<&str> = all_containers
        .get(attrs.as_str())
        .unwrap()
        .iter()
        .map(|c| c.as_str())
        .collect();

    let mut to_inspect: VecDeque<&str> = VecDeque::new();
    for container in containers.clone() {
        to_inspect.push_back(container);
    }

    while !to_inspect.is_empty() {
        let parent = to_inspect.pop_front().unwrap();
        match all_containers.get(parent) {
            None => {}
            Some(parent_parents) => {
                parent_parents.iter().for_each(|pc| {
                    containers.insert(pc);
                    to_inspect.push_back(pc);
                });
            }
        }
    }

    return containers;
}

fn containings_of(attrs: &str, all_containings: &HashMap<String, &Vec<ContainedBag>>) -> i32 {
    let contained = *all_containings.get(attrs).unwrap();

    return contained
        .iter()
        .map(|kid| kid.n + kid.n * containings_of(&kid.attrs, all_containings))
        .sum::<i32>();
}

fn to_bag(spec: &str) -> Bag {
    let parts = spec.split(" contain ").collect::<Vec<&str>>();

    let container_attrs = parts[0].replace(" bags", "");

    let contained_spec = parts[1];

    // ending bags
    if contained_spec.eq("no other bags.") {
        return Bag {
            attrs: container_attrs,
            contained_bags: Vec::new(),
        };
    }

    let _contained_bags = contained_spec
        .replace(".", "")
        .split(", ")
        .map(|contained_bag| {
            let bag_spec = contained_bag.replace(" bags", "").replace(" bag", "");
            let index_spc = bag_spec.find(" ").unwrap();
            let (n, spec) = bag_spec.split_at(index_spc);
            let nb = n.parse::<i32>().unwrap();
            let bag_spec = spec.trim_start();
            return ContainedBag {
                n: nb,
                attrs: String::from(bag_spec),
            };
        })
        .collect::<Vec<ContainedBag>>();

    return Bag {
        attrs: container_attrs,
        contained_bags: _contained_bags,
    };
}

fn attr_to_containers(bags: &Vec<Bag>) -> HashMap<String, HashSet<String>> {
    let mut containers: HashMap<String, HashSet<String>> = HashMap::new();

    for bag in bags.iter() {
        for contained_bag in bag.contained_bags.iter() {
            let cur_containers = containers
                .entry(contained_bag.attrs.clone())
                .or_insert_with(|| HashSet::new());

            cur_containers.insert(bag.attrs.clone());
        }
    }

    return containers;
}

fn attrs_to_containing(bags: &Vec<Bag>) -> HashMap<String, &Vec<ContainedBag>> {
    let mut bagz: HashMap<String, &Vec<ContainedBag>> = HashMap::new();

    for bag in bags {
        bagz.entry(bag.attrs.clone())
            .or_insert_with(|| &bag.contained_bags);
    }

    return bagz;
}
