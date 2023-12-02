use std::{collections::HashMap, f64::INFINITY, fmt, str::FromStr};
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

struct Assignment {
    not: bool,
    var: String,
    set: String,
}

enum Operator {
    AND,
    OR,
}
struct LHS {
    assignment_a: Assignment,
    operator: Operator,
    assignment_b: Assignment,
}

struct Rule {
    lhs: LHS,
    rhs: Assignment,
}

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

fn choose_name_from_map<T>(
    map: &HashMap<String, T>,
    exclude_fn: impl Fn(&String) -> bool,
) -> String {
    let mut i = 1_u32;
    let mut variable_names = Vec::new();
    for (variable_name, _) in map {
        if exclude_fn(variable_name) {
            continue;
        }
        variable_names.push(variable_name);
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
    let mut rule_variables = Vec::new();
    let mut rule_nots = Vec::new();
    let mut rule_sets = Vec::new();
    let mut operator = Operator::AND;
    for i in 1..=3 {
        let var_pool = if i == 3 {
            &output_variables
        } else {
            &input_variables
        };
        let var_name = choose_name_from_map(var_pool, |_| false);
        rule_variables.push(var_name);
        println!("1. Is\n2. Is not");
        let not_choice = input_in_range(1_u32, 2_u32);
        rule_nots.push(not_choice == 2);
        let set_name = choose_name_from_map(var_pool, |_| false);
        rule_sets.push(set_name);
        if i == 1 {
            println!("1. And\n2. Or");
            let operator_choice = input_in_range(1_u32, 2_u32);
            operator = if operator_choice == 1 {
                Operator::AND
            } else {
                Operator::OR
            };
        }
        if i == 2 {
            println!("==>")
        }
    }
    let rhs = Assignment {
        not: rule_nots.pop().unwrap(),
        var: rule_variables.pop().unwrap(),
        set: rule_sets.pop().unwrap(),
    };
    let assignment_b = Assignment {
        not: rule_nots.pop().unwrap(),
        var: rule_variables.pop().unwrap(),
        set: rule_sets.pop().unwrap(),
    };
    let assignment_a = Assignment {
        not: rule_nots.pop().unwrap(),
        var: rule_variables.pop().unwrap(),
        set: rule_sets.pop().unwrap(),
    };
    let lhs = LHS {
        assignment_a,
        assignment_b,
        operator,
    };
    rules.push(Rule { lhs, rhs });
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
            if line.slope == INFINITY {
                return 1.0;
            }
            return line.slope * crisp_value + line.y_intercept;
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

fn main() {
    println!("Hello, world!");
}
