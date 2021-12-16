use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};
use itertools::Itertools;

type DataType = Vec<(String, String)>;

fn task1(input: &DataType) -> u64
{
    dfs1(input, vec![], "start".to_string())
}

fn dfs1(input: &DataType, way: Vec<String>, element: String) -> u64
{
    // We are only allowed to visit small caves once
    if element.chars().next().unwrap().is_lowercase() && way.contains(&element)
    {
        return 0;
    }

    // Add the current node to my way
    let mut my_way = way;
    my_way.push(element.clone());

    // We found the end!
    if element == "end"
    {
        //println!("{}", my_way.join(","));
        return 1;
    }

    // Check all the neighbours
    input.iter().filter(|(a,_)| a == &element).map(|(_,b)| dfs1(input, my_way.clone(), b.clone())).sum()
}




fn task2(input: &DataType) -> u64
{
    dfs2(input, vec![], "start".to_string())

}

fn dfs2(input: &DataType, way: Vec<String>, element: String) -> u64
{
    // We are only allowed to visit small caves once, except one...
    if element.chars().next().unwrap().is_lowercase()
    {
        // We can only visit start once
        if element == "start".to_string() && way.contains(&element)
        {
            return 0;
        }

        // If the way already contains the current element, we have to check if a cave was already visited twice...
        if way.contains(&element) && ! way.iter().filter(|&s| s.chars().next().unwrap().is_lowercase()).all_unique()
        {
            return 0;
        }
    }

    // Add the current node to my way
    let mut my_way = way;
    my_way.push(element.clone());

    // We found the end!
    if element == "end"
    {
        //println!("{}", my_way.join(","));
        return 1;
    }

    // Check all the neighbours
    input.iter().filter(|(a,_)| a == &element).map(|(_,b)| dfs2(input, my_way.clone(), b.clone())).sum()
}



static DATA: &str = "ln-nr
ln-wy
fl-XI
qc-start
qq-wy
qc-ln
ZD-nr
qc-YN
XI-wy
ln-qq
ln-XI
YN-start
qq-XI
nr-XI
start-qq
qq-qc
end-XI
qq-YN
ln-YN
end-wy
qc-nr
end-nr";

fn prepare_data (data: &str) -> DataType
{
    data.split_ascii_whitespace().flat_map(|s|
        {
            if let Some((a, b)) = s.split("-").collect_tuple()
            {
            [(a.to_string(),b.to_string()),(b.to_string(),a.to_string())]
            }
            else {
                panic!("Couldn't split the line...");
            }
        }
    ).collect_vec()
}

// Normal setup below

fn main() {
    println!("Day 5");
    let input = print_timed_execute(|| prepare_data(DATA), "Data prep") ;
    print_result_timed_execute(||task1(&input), "Task1");
    print_result_timed_execute(||task2(&input), "Task2");
}

#[cfg(test)]
mod tests
{

    static TESTDATA1: &str =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

static TESTDATA2: &str =
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

static TESTDATA3: &str =
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA1)), 10);
        assert_eq!(super::task1(&super::prepare_data(TESTDATA2)), 19);
        assert_eq!(super::task1(&super::prepare_data(TESTDATA3)), 226);

    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA1)), 36);
        assert_eq!(super::task2(&super::prepare_data(TESTDATA2)), 103);
        assert_eq!(super::task2(&super::prepare_data(TESTDATA3)), 3509);

    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 4773)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 116985)
    }

}