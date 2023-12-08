use std::{collections::HashMap, f64::INFINITY, fmt, str::FromStr};

use multimap::MultiMap;
#[macro_use]
extern crate scan_fmt;

struct Range {
    begin: f64,
    end: f64,
}

struct Line {
    slope: f64,
    y_intercept: f64,
    x_range: Range,
}

#[derive(Clone)]
struct Assignment {
    not: bool,
    var: String,
    set: String,
}

#[derive(Clone)]
enum Operator {
    AND,
    OR,
    THEN,
}
#[derive(Clone)]
enum Token {
    Assignment(Assignment),
    Operator(Operator),
    Value(f64),
}

type Rule = Vec<Token>;

type VariableMap = HashMap<String, HashMap<String, Vec<Line>>>;

fn get_input<T>() -> T
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    loop {
        if let Ok(input) = scanln_fmt!("{}", T) {
            return input;
        } else {
            println!("Invalid input, try again");
        }
    }
}

fn input_in_range<T>(start: T, end: T) -> T
where
    T: FromStr + PartialOrd,
    T::Err: fmt::Debug,
{
    loop {
        let input = get_input::<T>();
        if input < start || input > end {
            println!("Input out of range");
        } else {
            return input;
        }
    }
}

fn choose_name_from_map<T>(map: &HashMap<String, T>) -> String {
    let mut variable_names = Vec::new();
    for (variable_name, _) in map {
        variable_names.push(variable_name);
    }
    variable_names.sort();
    let mut i = 1_u32;
    for variable_name in &variable_names {
        println!("{}. {}", i, variable_name);
        i += 1;
    }
    let number: u32 = input_in_range(1_u32, i - 1) - 1;
    variable_names[number as usize].to_owned()
}

fn add_rule(input_variables: &VariableMap, output_variables: &VariableMap, rules: &mut Vec<Rule>) {
    if input_variables.len() == 0 || output_variables.len() == 0 {
        println!("Insufficient input and/or output variables to create a rule");
        return;
    }
    let mut rule: Rule = Vec::new();
    let mut in_lhs = true;
    let operators = vec![Operator::AND, Operator::OR, Operator::THEN];
    let mut i = 1_u32;
    loop {
        let turn = i % 2;
        if turn == 1 {
            let var_pool = if in_lhs {
                &input_variables
            } else {
                &output_variables
            };
            let var_name = choose_name_from_map(var_pool);
            println!("1. Is\n2. Is not");
            let not_choice = input_in_range(1_u32, 2_u32);
            let not = not_choice == 2;
            let set_name = choose_name_from_map(var_pool.get(&var_name).unwrap());
            rule.push(Token::Assignment(Assignment {
                not,
                var: var_name,
                set: set_name,
            }));
        } else if turn == 0 {
            if in_lhs {
                println!("1. And\n2. Or\n3. Then");
                let operator_choice = input_in_range(1_usize, 3_usize);
                let operator = &operators[operator_choice - 1];
                rule.push(Token::Operator(operator.clone()));
                if matches!(operator, Operator::THEN) {
                    in_lhs = false;
                }
            } else {
                println!("1. ,\n2. Done");
                let operator_choice = input_in_range(1_u32, 2_u32);
                if operator_choice == 2 {
                    break;
                }
            }
        }
        i += 1;
    }
    rules.push(rule);
}

fn compute_y(line: &Line, value: f64) -> f64 {
    if line.slope == INFINITY {
        return 1.0;
    }
    return line.slope * value + line.y_intercept;
}

fn calculate_fuzzy_value_for_variable(
    variables: &VariableMap,
    variable_name: &String,
    set_name: &String,
    crisp_value: f64,
) -> f64 {
    let lines = variables.get(variable_name).unwrap().get(set_name).unwrap();
    for line in lines {
        if crisp_value >= line.x_range.begin && crisp_value <= line.x_range.end {
            return compute_y(&line, crisp_value);
        }
    }
    return 0.0;
}

fn add_variable(variables: &mut VariableMap) {
    println!("Variable name");
    let variable_name: String = get_input();
    variables.insert(variable_name.clone(), HashMap::new());
    println!("Number of sets (1 - 10000)");
    let no_sets: u32 = input_in_range(1_u32, 10000);
    for i in 1..=no_sets {
        println!("Set {} name", i);
        let set_name: String = get_input();
        println!("Number of points (3: triangular, 4: trapezoidal)");
        let no_points: u32 = input_in_range(3, 4);
        let mut xs = Vec::new();
        for j in 1..=no_points {
            println!("Point {}", j);
            let point: f64 = get_input();
            xs.push(point);
        }
        let mut lines = Vec::new();
        let mut ys = Vec::new();
        ys.push(0_f64);
        for _ in 2..no_points {
            ys.push(1_f64);
        }
        ys.push(0_f64);
        for j in 1..no_points as usize {
            let slope = (ys[j] - ys[j - 1]) / (xs[j] - xs[j - 1]);
            lines.push(Line {
                slope,
                y_intercept: ys[j] - slope * xs[j],
                x_range: Range {
                    begin: xs[j - 1],
                    end: xs[j],
                },
            });
        }
        variables
            .get_mut(&variable_name)
            .unwrap()
            .insert(set_name, lines);
    }
}

fn get_value_from_token(token: &Token) -> f64 {
    let val = match token {
        Token::Value(v) => v.to_owned(),
        _ => panic!("Parse error"),
    };
    val
}

fn compute_result_of_two_tokens(
    tokens: &mut Rule,
    mut mid_index: usize,
    func: fn(f64, f64) -> f64,
) {
    mid_index += 1;
    let val1 = get_value_from_token(&tokens[mid_index]);
    mid_index -= 2;
    let val2 = get_value_from_token(&tokens[mid_index]);
    let res = func(val1, val2);
    tokens.splice(mid_index..(mid_index + 3), vec![Token::Value(res)]);
}

fn calculate_crisp_output(
    input_variables: &VariableMap,
    output_variables: &VariableMap,
    rules: &Vec<Rule>,
) {
    if rules.len() == 0 {
        println!("Need at least one rule to calculate crisp outputs");
        return;
    }
    let mut input_crisp_values: HashMap<String, f64> = HashMap::new();
    let mut output_fuzzy_values = HashMap::new();
    for rule in rules {
        let mut tokens_list = rule.clone();
        let mut idx = 0;
        while !matches!(&tokens_list[idx], Token::Operator(Operator::THEN)) {
            match &tokens_list[idx] {
                Token::Assignment(assignment) => {
                    let crisp = if input_crisp_values.contains_key(&assignment.var) {
                        input_crisp_values.get(&assignment.var).unwrap().clone()
                    } else {
                        println!("{} crisp value", &assignment.var);
                        get_input()
                    };
                    input_crisp_values.insert(assignment.var.clone(), crisp);
                    let mut fuzzy = calculate_fuzzy_value_for_variable(
                        &input_variables,
                        &assignment.var,
                        &assignment.set,
                        crisp,
                    );
                    if assignment.not {
                        fuzzy = 1.0 - fuzzy;
                    }
                    tokens_list.splice(idx..(idx + 1), vec![Token::Value(fuzzy)]);
                }
                _ => {}
            }
            idx += 1;
        }
        idx = 0;
        while !matches!(&tokens_list[idx], Token::Operator(Operator::THEN)) {
            if matches!(&tokens_list[idx], Token::Operator(Operator::AND)) {
                compute_result_of_two_tokens(&mut tokens_list, idx, f64::min);
                idx -= 1;
            }
            idx += 1;
        }
        idx = 0;
        while !matches!(&tokens_list[idx], Token::Operator(Operator::THEN)) {
            if matches!(&tokens_list[idx], Token::Operator(Operator::OR)) {
                compute_result_of_two_tokens(&mut tokens_list, idx, f64::max);
                idx -= 1;
            }
            idx += 1;
        }
        let fuzzy_result = get_value_from_token(&tokens_list[idx - 1]);
        while idx != tokens_list.len() {
            match &tokens_list[idx] {
                Token::Assignment(assignment) => {
                    let mut assignment_result = fuzzy_result;
                    if assignment.not {
                        assignment_result = 1.0 - assignment_result;
                    }
                    if !output_fuzzy_values.contains_key(&assignment.var) {
                        output_fuzzy_values.insert(assignment.var.clone(), MultiMap::new());
                    }
                    output_fuzzy_values
                        .get_mut(&assignment.var)
                        .unwrap()
                        .insert(assignment.set.clone(), assignment_result);
                }
                _ => {}
            }
            idx += 1;
        }
    }

    for (var_name, set) in &output_fuzzy_values {
        let mut numerator = 0_f64;
        let mut denominator = 0_f64;
        for (set_name, fuzzy_values) in set.iter_all() {
            let mut centroid = 0_f64;
            let lines = output_variables
                .get(var_name)
                .unwrap()
                .get(set_name)
                .unwrap();
            for line in lines {
                centroid += line.x_range.begin;
            }
            centroid += lines.last().unwrap().x_range.end;
            centroid /= (lines.len() + 1) as f64;
            for fuzzy_value in fuzzy_values {
                numerator += centroid * fuzzy_value;
                denominator += fuzzy_value;
            }
        }
        let crisp_value = numerator / denominator;
        let mut set_name = "Invalid set";
        let mut max_y = -1_f64;
        for (s, lines) in output_variables.get(var_name).unwrap() {
            for line in lines {
                if crisp_value >= line.x_range.begin && crisp_value <= line.x_range.end {
                    let prev_max_y = max_y;
                    max_y = f64::max(max_y, compute_y(&line, crisp_value));
                    if max_y != prev_max_y {
                        set_name = s;
                    }
                }
            }
        }
        println!("{}: {} ({})", var_name, crisp_value, set_name);
    }
}

fn main() {
    let mut input_variables = HashMap::new();
    let mut output_variables = HashMap::new();
    let mut rules = Vec::new();
    loop {
        println!(
            "1. Add input variable
2. Add output variable
3. Add rule
4. Calculate crisp values for output variables"
        );
        let choice = input_in_range(1_u32, 4);
        match choice {
            1 => add_variable(&mut input_variables),
            2 => add_variable(&mut output_variables),
            3 => add_rule(&input_variables, &output_variables, &mut rules),
            4 => calculate_crisp_output(&input_variables, &output_variables, &rules),
            _ => println!("Invalid choice"),
        }
    }
}
