pub fn display(input: &str, sep: &str) -> Vec<String> {
    let digits = vec![
        vec![
            "  ###  ",
            " #   # ",
            "#     #",
            "#     #",
            "#     #",
            " #   # ",
            "  ###  ",
        ],
        vec![
            "   #   ",
            "  ##   ",
            " # #   ",
            "   #   ",
            "   #   ",
            "   #   ",
            " ##### ",
        ],
        vec![
            " ##### ",
            "#     #",
            "      #",
            " ##### ",
            "#      ",
            "#      ",
            "#######",
        ],
        vec![
            " ##### ",
            "#     #",
            "      #",
            " ##### ",
            "      #",
            "#     #",
            " ##### ",
        ],
        vec![
            "#      ",
            "#    # ",
            "#    # ",
            "#    # ",
            "###### ",
            "     # ",
            "     # ",
        ],
        vec![
            "#######",
            "#      ",
            "#      ",
            "###### ",
            "      #",
            "#     #",
            " ##### ",
        ],
        vec![
            " ##### ",
            "#     #",
            "#      ",
            "###### ",
            "#     #",
            "#     #",
            " ##### ",
        ],
        vec![
            "#######",
            "     # ",
            "    #  ",
            "   #   ",
            "  #    ",
            " #     ",
            "#      ",
        ],
        vec![
            " ##### ",
            "#     #",
            "#     #",
            " ##### ",
            "#     #",
            "#     #",
            " ##### ",
        ],
        vec![
            " ##### ",
            "#     #",
            "#     #",
            " ######",
            "      #",
            "#     #",
            " ##### ",
        ],
    ];
    let mut output = vec![String::new(); 7];

    for c in input.chars() {
        match c.to_digit(10) {
            None => {
                for line in &mut output {
                    line.push_str(&format!(" {} ", sep))
                }
            },
            Some(digit) => {
                for (i, d) in digits[digit as usize].iter().enumerate() {
                    output[i].push_str(&format!(" {}", d));
                }
            },
        }
    }
    output
}