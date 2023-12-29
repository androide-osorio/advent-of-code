pub fn main() {
    println!("Part 2!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_1() {
        let input1 = r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_example_2() {
        let input2 = r#"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "#;
        assert_eq!(2 + 2, 4);
    }
}
