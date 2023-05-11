pub fn display(input: &str, sep: &str, is_large: bool) -> Vec<String> {

    let digits = digits(is_large);
    let mut output = vec![String::new(); digits[0].len()];

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

pub fn padding(num: usize, length: usize) -> String {
    let mut output = num.to_string();
    while output.len() < length {
        output = format!("0{}", output);
    }
    output
}


fn digits(is_large: bool) -> Vec<Vec<&'static str>> {
    if is_large { 
        vec![
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
            "      #",
            "      #",
            " ##### ",
            "      #",
            "      #",
            " ##### ",
        ],
        vec![
            "#     #",
            "#     #",
            "#     #",
            "#######",
            "      #",
            "      #",
            "      #",
        ],
        vec![
            "#######",
            "#      ",
            "#      ",
            " ##### ",
            "      #",
            "      #",
            " ##### ",
        ],
        vec![
            " ##### ",
            "#      ",
            "#      ",
            " ##### ",
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
    ]
    } else {
 vec![
        vec![
            "###",
            "# #",
            "# #",
            "# #",
            "###",
        ],
        vec![
            "  #",
            " ##",
            "# #",
            "  #",
            "###",
        ],
        vec![
            "###",
            "  #",
            "###",
            "#  ",
            "###",
        ],
        vec![
            "###",
            "  #",
            "###",
            "  #",
            "###",
        ],
        vec![
            "# #",
            "# #",
            "###",
            "  #",
            "  #",
        ],
        vec![
            "###",
            "#  ",
            "###",
            "  #",
            "###",
        ],
        vec![
            " # ",
            "#  ",
            "###",
            "# #",
            "###",
        ],
        vec![
            "###",
            "  #",
            " # ",
            " # ",
            " # ",
        ],
        vec![
            "###",
            "# #",
            "###",
            "# #",
            "###",
        ],
        vec![
            "###",
            "# #",
            "###",
            "  #",
            "###",
        ],
]
    }
}