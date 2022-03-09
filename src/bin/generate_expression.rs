use rand::{ distributions::Uniform, Rng };

enum State {
    Operand,
    Operator,
}

struct Options {
    probability_multiplier: f32,
    negative_probability: f32,
    begin_group_probability: f32,
    group_probability_multiplier: f32,
    group_probability_multiplier_modifier: f32,
    number_size: (u8, u8),
}

fn generate_number(min: u8, max: u8) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Uniform::new(0, 10))
        .take(rand::thread_rng().sample(&Uniform::new(min, max + 1)) as usize)
        .map(|n| (n + 48) as u8 as char)
        .collect();
    s
}

fn generate_operator() -> char {
    match rand::thread_rng().sample(&Uniform::new(0, 5)) {
        0 => '+',
        1 => '-',
        2 => '*',
        3 => '/',
        4 => '%',
        _ => panic!("unexpected error"),
    }
}

fn randf() -> f32 {
    rand::random::<f32>()
}

fn generate(options: Options, state: State, probability: f32, probability_multiplier: f32, begin_group_probability: f32) -> String {
    match state {
        State::Operand => {
            if randf() < begin_group_probability {
                let group_probability_multiplier = options.group_probability_multiplier;
                let begin_group_probability = options.begin_group_probability;
                let group_probability_multiplier_modifier = options.group_probability_multiplier_modifier;

                format!("({})", generate(options, State::Operand, 1.0, group_probability_multiplier, begin_group_probability * group_probability_multiplier_modifier))
            } else {
                let (min, max) = options.number_size;
                let number = generate_number(min, max);
                let mut s = String::new();
                
                if randf() < options.negative_probability {
                    s.push('-');
                }
                s.push_str(&number);
                if randf() < probability {
                    s.push_str(&generate(options, State::Operator, probability * probability_multiplier, probability_multiplier, begin_group_probability));
                }
                s
            }
        },
        State::Operator => {
            let operator = generate_operator();
            let mut s = String::new();

            s.push(operator);
            s.push_str(&generate(options, State::Operand, probability, probability_multiplier, begin_group_probability));
            s
        },
    }
}

fn main() {
    let options = Options {
        probability_multiplier: 0.99,
        //probability_multiplier: 0.6,
        negative_probability: 0.2,
        begin_group_probability: 0.4,
        group_probability_multiplier: 0.9,
        group_probability_multiplier_modifier: 0.8,
        //number_size: (1, 4),
        //number_size: (12, 24),
        //number_size: (40, 60),
        number_size: (80, 100),
    };
    let probability_multiplier = options.probability_multiplier;
    let begin_group_probability = options.begin_group_probability;
    let expression = generate(options, State::Operand, 1.0, probability_multiplier, begin_group_probability);
    let result = String::from_utf8(
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("echo '{}' | bc", expression))
            .output()
            .expect("")
            .stdout
    ).expect("");
    let result = result.trim_end();

    assert_ne!(result, "");
    println!("assert_eq!(format!(\"{{}}\", eval(\"{}\")), \"{}\");", expression, result);
}
