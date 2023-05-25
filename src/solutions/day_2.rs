struct Present(u32, u32, u32);

impl Present {
    fn new(dimensions: &str) -> Self {
        let mut dimensions: Vec<_> = dimensions.trim()
            .split("x")
            .map(|d| d.parse().unwrap())
            .collect();

        dimensions.sort();

        Present(dimensions[0], dimensions[1], dimensions[2])
    }

    fn surface_area(&self) -> u32 {
        2 * (self.0 * (self.1 + self.2) + self.1 * self.2)
    }

    fn wrapping_paper(&self) -> u32 {
        let smallest_side = self.0 * self.1;

        self.surface_area() + smallest_side
    }

    fn ribbon(&self) -> u32 {
        let main_ribbon = 2 * (self.0 + self.1);

        let bow = self.0 * self.1 * self.2;

        main_ribbon + bow
    }
}

fn parse_presents(input: &str) -> Vec<Present> {
    input.split('\n')
        .filter(|l| !l.is_empty())
        .map(Present::new)
        .collect()
}

pub fn solve_part_1(input: &str) -> u32 {
    parse_presents(input).into_iter()
        .map(|p| p.wrapping_paper())
        .sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    parse_presents(input).into_iter()
        .map(|p| p.ribbon())
        .sum()
}

#[test]
fn test_wrapping_paper_1() {
    let present = Present::new("2x3x4");
    assert_eq!(present.wrapping_paper(), 58);
}

#[test]
fn test_wrapping_paper_2() {
    let present = Present::new("1x1x10");
    assert_eq!(present.wrapping_paper(), 43);
}
