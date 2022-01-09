fn main() {
    let rules = "";
}

const EXAMPLE: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
const FULL: &str = include_str!("input.txt");

use itertools::Itertools;
// how many bags can contain at least one shiny gold bag?
use multimap::MultiMap;
use std::fmt;

pub type BagSpec<'a> = (&'a str, &'a str);

// K can contain V.0 of V.1
pub type Rules<'a> = MultiMap<BagSpec<'a>, (usize, BagSpec<'a>)>;

struct FormattedRules<'a>(Rules<'a>);

// format much closer to the puzzle input
impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bags can contain ", k.0, k.1)?;
            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "{} {} {} {}",
                        v.0,
                        v.1 .0,
                        v.1 .1,
                        if v.0 == 1 { "bag" } else { "bags" }
                    )?;
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Default::default();

    peg::parser! {
        pub(crate) grammar parser() for str {

            pub(crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = spec:bag_spec() " contain " rules:rules() {
                    if let Some(rules) = rules {
                        for rule in rules {
                            r.insert(spec, rule)
                        }
                    }
                }

            rule bag_spec() -> BagSpec<'input>
                = adjective:name() " " color:name() " bag" "s"? { (adjective, color)}

            rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
             = rules:rule1()+ { Some(rules) }
             / "no other bags" { None }

            /// rule followed by an optional comma and space
            rule rule1() -> (usize, BagSpec<'input>)
                = r:rule0() ", "? { r }

            /// a single rule
            rule rule0() -> (usize, BagSpec<'input>)
                = quantity:number() " " spec:bag_spec() { (quantity, spec) }

            rule number() -> usize
                = e:$(['0'..='9']+) { e.parse().unwrap() }

            /// A sequence of non-whitespace characters
            rule name() -> &'input str
                = $((!whitespace()[_])*)

            /// Spaces, tabs, CR and LF
            rule whitespace()
                = [' ' | '\t' | '\r' | '\n' ]
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}

fn subgraph_contains(graph: &Rules<'_>, root: &(&str, &str), needle: &(&str, &str)) -> bool {
    // if let Some(neighbors) = graph.get_vec(root) {
    //     for (_quantity, neighbor) in neighbors {
    //         if neighbor == needle || subgraph_contains(graph, neighbor, needle) {
    //             return true;
    //         }
    //     }
    // }
    // false

    // or a fancy-pants iterator method:
    // graph.get_vec(root)
    //     .map(|v| {
    //         v.iter().any(|(_quantity, neighbor)| {
    //             neighbor == needle || subgraph_contains(graph, neighbor, needle)
    //         })
    //     }).unwrap_or_default()

    // or a fancier, flatter iterator method:
    graph
        .get_vec(root)
        .into_iter()
        .flatten()
        .any(|(_quantity, neighbor)| {
            neighbor == needle || subgraph_contains(graph, neighbor, needle)
        })
}

fn calc(s: &str) -> usize {
    let rules = parse_rules(s);
    let needle = &("shiny", "gold");
    let colors_that_contain_shiny_gold = rules
        .keys()
        // ignore the needle in keys
        .filter(|&k| k != needle)
        .filter(|&k| subgraph_contains(&rules, k, needle));
    colors_that_contain_shiny_gold.count()
}

// Ok, but we want to try walking *upward* from the chosen color

fn reverse_graph<'a>(graph: &Rules<'a>) -> Rules<'a> {
    // let mut reverse: Rules = Default::default();
    // for (&node, neighbors) in graph.iter_all() {
    //     for &(quantity, neighbor) in neighbors {
    //         reverse.insert(neighbor, (quantity, node))
    //     }
    // }
    // reverse

    // - or collect into a MultiMap -
    graph
        .iter_all()
        .map(|(&node, neighbors)| {
            neighbors
                .iter()
                .map(move |&(quantity, neighbor)| (neighbor, (quantity, node)))
        })
        .flatten()
        .collect()
}

fn walk_subgraph<'a>(graph: &Rules<'a>, root: &(&str, &str)) -> Vec<(&'a str, &'a str)> {
    let mut res: Vec<_> = Default::default();
    if let Some(neighbors) = graph.get_vec(root) {
        for &(_quantity, neighbor) in neighbors {
            res.push(neighbor);
            res.extend(walk_subgraph(graph, &neighbor));
        }
    }
    res
}

fn walk_subgraph_mut<'a>(
    graph: &Rules<'a>,
    root: &(&str, &str),
    res: &mut Vec<(&'a str, &'a str)>,
) {
    if let Some(neighbors) = graph.get_vec(root) {
        for &(_quantity, neighbor) in neighbors {
            res.push(neighbor);
            res.extend(walk_subgraph(graph, &neighbor));
        }
    }
}

// this 😱 is a 😱 dense 😱 lifetime 😱 set
fn walk_subgraph_box<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (&'elems str, &'elems str)> + 'iter> {
    // why is this even in a Box?
    // https://fasterthanli.me/articles/recursive-iterators-rust
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(_quantity, neighbor)| {
                std::iter::once(neighbor).chain(walk_subgraph_box(graph, &neighbor))
            })
            .flatten(),
    )
}

fn cooler_calc(s: &str) -> usize {
    let rules = parse_rules(s);
    let rev_rules = reverse_graph(&rules);

    let needle = ("shiny", "gold");
    let answer = walk_subgraph_box(&rev_rules, &needle).unique().count();
    answer
}

// Part 2! Counting bags needed to fill a shiny gold bag
fn walk_subgraph_with_quantities<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
    // vvvv now returning quantity
) -> Box<dyn Iterator<Item = (usize, (&'elems str, &'elems str))> + 'iter> {
    // why is this even in a Box?
    // https://fasterthanli.me/articles/recursive-iterators-rust
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            // basically the same as above though
            .map(move |&n| std::iter::once(n).chain(walk_subgraph_with_quantities(graph, &n.1)))
            .flatten(),
    )
}

fn bag_quantities<'iter>(
    graph: &'iter Rules<'iter>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(qt, n)| {
                std::iter::once(qt).chain(bag_quantities(graph, &n).map(move |x| x * qt))
            })
            .flatten(),
    )
}

fn calc_pt2(s: &str) -> usize {
    let rules = parse_rules(s);
    let root = ("shiny", "gold");
    let answer: usize = bag_quantities(&rules, &root).sum();
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let s = EXAMPLE;
        assert_eq!(calc(s), 4);
    }

    #[test]
    fn full_input() {
        let s = FULL;
        assert_eq!(calc(s), 335);
    }

    #[test]
    fn cooler_example_input() {
        let s = EXAMPLE;
        assert_eq!(cooler_calc(s), 4);
    }

    #[test]
    fn cooler_full_input() {
        let s = FULL;
        assert_eq!(cooler_calc(s), 335);
    }

    #[test]
    fn calc_part2_example_input() {
        let s = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(calc_pt2(s), 126);
    }

    #[test]
    fn calc_part2_full_input() {
        let s = FULL;
        assert_eq!(calc_pt2(s), 2431);
    }
}
