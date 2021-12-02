use advent_of_code_2021::read_file_into_line_array;
use itertools::Itertools;

fn main() {
    let mut values: Vec<u64> = read_file_into_line_array("input/day10.txt")
        .into_iter()
        .map(|s| s.parse::<u64>().expect("Invalid command found"))
        .collect();

    values.push(values.iter().max().unwrap() + 3);
    values.insert(0, 0);
    values.sort();

    let counts = values.iter().tuple_windows().map(|(lower, higher)| higher-lower).counts();

    println!("Task 1: {:?}, multiple of 3s by 1s: {:?}", counts, counts[&1u64] * counts[&3u64] );





    let mut graph : Vec<(u64, u64)> = Vec::new();

    // Get all elements that can connect as edges
    for (one, two, three, four) in values.iter().tuple_windows()
    {
        if two-one < 4
        {
            graph.push((*one, *two));
        }
        if three-one < 4
        {
            graph.push((*one, *three));
        }
        if four-one < 4
        {
            graph.push((*one, *four));
        }
    }

    // Split the graph into sub-graphs, wherever there is only one connection
    let mut graphs : Vec<Vec<(u64, u64)>> = Vec::new();
    let mut index_start_segment = 0usize;

    for index_end_segment in 1..graph.len()
    {
        let (from, to) = graph[index_end_segment];
        if graph.iter().filter(|(from_filter, _)| from_filter == &from).count() == 1 && graph.iter().filter(|(_, to_filter)| to_filter == &to).count() == 1
        {
            let slice = &graph[index_start_segment..index_end_segment];
            if slice.len() != 0
            {
                graphs.push(slice.to_vec());
            }

            index_start_segment = index_end_segment;
        }
    }

    // In the resulting graph, do recursive backtracing
    fn path_count(graph: &Vec<(u64, u64)>, source: &u64, target: &u64) -> usize
    {
        if source == target
        {
            return 1usize;
        }
        else
        {
            graph.iter().filter(|(g_source, _)| g_source == source).fold(0usize, |count, (_, g_target)| count + path_count(graph, g_target, target))
        }

    }

    let possibilities_per_graph = graphs.iter().map(|graph|
        {
            let source = graph.iter().map(|(min, _)| min).min().unwrap();
            let target = graph.iter().map(|(_, max)| max).max().unwrap();

            let numways = path_count(graph, source, target);

            println!("Graph with length {:?}, from {:?} to {:?} with {:?} possible ways: {:?}", graph.len(), source, target, numways ,graph);

            numways
            // TODO find possibilities per graph!

        }
    );

    println!("Task 2: {:?}", &possibilities_per_graph.product::<usize>() );
}
