use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};

type DataType = Packet;



// https://codereview.stackexchange.com/a/228871
fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}
fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[derive(Debug)]
enum Packet
{
    Literal{version: u8, value: u64},
    Operator{version: u8, op_type: u8, children: Vec<Packet>}
}

impl Packet {
    fn get_version_sum(&self) -> u64
    {
        match self {
            Packet::Literal { version, value: _ } => *version as u64,
            Packet::Operator { version, op_type: _, children } => (*version as u64) + children.iter().map(|f| f.get_version_sum()).sum::<u64>(),
        }
    }

    fn calc(&self) -> u64
    {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operator { version: _, op_type, children } => 
            {
                match op_type
                {
                    0 => children.iter().map(|child| child.calc()).sum(),
                    1 => children.iter().map(|child| child.calc()).product(),
                    2 => children.iter().map(|child| child.calc()).min().unwrap(),
                    3 => children.iter().map(|child| child.calc()).max().unwrap(),
                    5 => if children[0].calc() > children[1].calc() {1} else {0},
                    6 => if children[0].calc() < children[1].calc() {1} else {0},
                    7 => if children[0].calc() == children[1].calc() {1} else {0},
                    _ => panic!()
                }
            },
        }
    }
}

fn task1(input: &DataType) -> u64
{
    input.get_version_sum()
}


fn task2(input: &DataType) -> u64
{
    input.calc()
}


static DATA: &str = include_str!("../../input/day16.txt");

fn prepare_data (data: &str) -> DataType
{
    let binstring = convert_to_binary_from_hex(data);
    let (result, _) = parse_string(binstring.as_str());
    result
}

fn parse_string(data: &str) -> (DataType /* The result */, usize /* Consumed binary characters */)
{
    let packet_version = u8::from_str_radix(&data[..3], 2).unwrap();
    let packet_type = u8::from_str_radix(&data[3..6], 2).unwrap();

    let mut consumed_characters = 6usize;

    let result = match packet_type {
        4 => // Literal
        {
            let mut value: u64 = 0;
            while data.as_bytes()[consumed_characters] as char == '1'
            {
                value <<= 4;
                value += u64::from_str_radix(&data[consumed_characters+1..consumed_characters+5], 2).unwrap();
                consumed_characters += 5;
            }
            value <<= 4;
            value += u64::from_str_radix(&data[consumed_characters+1..consumed_characters+5], 2).unwrap();
            consumed_characters += 5;

            Packet::Literal{version: packet_version, value}
        }

        _ => // Operator
        {
            // Read the length type
            let len_type = data.as_bytes()[consumed_characters] as char;
            consumed_characters += 1;

            // This are the children...
            let mut children = vec![];

            // Read the length, depending on length type
            match len_type {
                '0' =>
                {
                    let len_chars = usize::from_str_radix(&data[consumed_characters..consumed_characters+15], 2).unwrap();
                    consumed_characters += 15;
                    let max_char = consumed_characters + len_chars;
                    while consumed_characters + 11 <= max_char
                    {
                        let (child, child_characters) = parse_string(&data[consumed_characters..]);
                        consumed_characters += child_characters;
                        children.push(child);
                    }

                }
                '1' =>
                {
                    let len_numchildren = usize::from_str_radix(&data[consumed_characters..consumed_characters+11], 2).unwrap();
                    consumed_characters += 11;

                    for _ in 0..len_numchildren
                    {
                        let (child, child_characters) = parse_string(&data[consumed_characters..]);
                        consumed_characters += child_characters;
                        children.push(child);
                    }
                }
                _ => panic!()
            };


            Packet::Operator{version: packet_version, op_type: packet_type, children}

        }
    };

    (result, consumed_characters)

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

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data("D2FE28")), 6);
        assert_eq!(super::task1(&super::prepare_data("8A004A801A8002F478")), 16);
        assert_eq!(super::task1(&super::prepare_data("620080001611562C8802118E34")), 12);
        assert_eq!(super::task1(&super::prepare_data("C0015000016115A2E0802F182340")), 23);
        assert_eq!(super::task1(&super::prepare_data("A0016C880162017C3686B18A3D4780")), 31);
    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data("C200B40A82")), 3);
        assert_eq!(super::task2(&super::prepare_data("04005AC33890")), 54);
        assert_eq!(super::task2(&super::prepare_data("880086C3E88112")), 7);
        assert_eq!(super::task2(&super::prepare_data("CE00C43D881120")), 9);
        assert_eq!(super::task2(&super::prepare_data("D8005AC2A8F0")), 1);
        assert_eq!(super::task2(&super::prepare_data("F600BC2D8F")), 0);
        assert_eq!(super::task2(&super::prepare_data("9C005AC2F8F0")), 0);
        assert_eq!(super::task2(&super::prepare_data("9C0141080250320F1802104A08")), 1);

    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 953)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 246225449979)
    }

}