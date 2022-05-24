/*
Yet Another Math Solver
 */
use std::borrow::Borrow;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::num::ParseFloatError;
use std::ops::Index;
use std::slice;
use std::string::ParseError;
use std::thread::current;
use crate::ClosedLevel::{complete, contained, unclosed};

static OPERATORS: [char; 5] = ['-', '+', '*', '/', '^'];

struct Solver{
    variables : Map<String, f64>,
    functions : Map<String, Statement>,
    history : Vec<String>
}

impl Solver{
    fn evaluate(statement : Statement) -> f64{
        0.0
    }
}
/**
* When parsing nodes,
*/
struct Value {
    /**
     * original string read by parser
     */
    org_str: String,
    /**
     * Numerical if it exists.
     */
    num_value : Option<f64>,
    /**
     * Character used for variable if it exists
     */
    variable : Option<String>,
    /**
     * Operator used for node left and right, only used for non-leaf nodes.
     */
    operator : Option<char>
}
impl Value{
    fn from(str : String) -> Value{
        //try to get a number value by parsing, if this fails we can move on to interpreting as var
        let num_value = match str.parse::<f64>(){
            Err(_) => {
                None
            }
            Ok(result) => {
                Option::from(Value{
                    org_str: str.clone(),
                    num_value: Option::from(result),
                    variable: None,
                    operator: None
                })
            }
        };
        //only check for operators if we only have one character
        if str.len() == 1 {
            for x in OPERATORS {
                if x.to_string() == str {
                    return Value{
                        org_str: str.clone(),
                        num_value: None,
                        variable: None,
                        operator : Option::from(str.clone().chars().nth(0).unwrap())
                    }
                }
            }
        }
        //otherwise it has to be a variable.
        Value{
            org_str: str.clone(),
            num_value: None,
            variable: Option::from(str.clone()),
            operator: None
        }
    }
}

/**
nodes are atomic unit, numbers, variables, transcendental/irresolvable functions, etc.
*/
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: Value
}
/**
Statements are trees of operators organized by order of operations
 */
struct Statement{
    statementString : String,
    root : Node
}
impl Statement {

}
#[derive(PartialEq)]
enum ClosedLevel {
    contained,
    complete,
    unclosed
}
fn closed(str : &str) -> ClosedLevel {
    let chars = str.chars();
    let mut is_complete = false;
    let mut counter = 0;
    for (index, char) in chars.into_iter().enumerate(){
        if char == '(' {
            if index == 0 {
                is_complete = true;
            }
            counter += 1;
        }
        if char == ')' && counter > 0 {
            counter -= 1;
            if counter == 0 && index != str.len()-1 {
                is_complete = false;
            }
        }else if char == ')' && counter == 0 {
            println!("unclosed");
            return unclosed;
        }
    }
    if counter > 0 {
        unclosed
    }else {
        if is_complete == true {
            complete
        }else {
            contained
        }
    }
}
fn is_closed_pos(str : &str, pos : usize) -> Result<bool, String>{
    //make sure usize is in str
    if pos >= str.len() {
        return Err("Invalid usize".to_string());
    }
    let mut chars : Vec<char> = str.chars().collect();
    //we know our statement is valid (hopefully) so we really only need to know if there is a (
    //before our position, and if it is followed by )
    let mut count = 0;
    for index in 0..pos {
        if chars[index] == ')'{
            count -= 1;
        }
        if chars[index] == '('{
            count += 1;
        }
    }
    if count == 0 {
        Ok(false)
    }else if count > 0 {
        Ok(true)
    }else {
        Err("un-contained statement!".to_string())
    }
}

fn evaluate(str: &str) -> Result<Node, String> {
    //first make sure our statement is closed
    println!("{}", str);
    let close_level = closed(str);
    if close_level == unclosed {
        return Err("un-contained statement!".to_string())
    }
    //we know here statement is closed, check if completely closed. if so, we can remove parenthesis
    if close_level == complete {
        return Ok(evaluate(&str[1..str.len()-1]).unwrap());
    }
    for op in OPERATORS {
        for (index, char) in str.chars().into_iter().enumerate() {
            //we ignore possible error with unwrap as we verified the string with closed in above statement.
            if op == char && !is_closed_pos(str, index).unwrap() {
                let left = &str[0..index];
                let right = &str[index + 1..str.len()];
                println!("{}, {}", left, right);
                return Ok(Node {
                    left: Option::from(Box::from(evaluate(left).unwrap())),
                    right: Option::from(Box::from(evaluate(right).unwrap())),
                    value: Value::from(op.to_string())
                });
            }
        }
    }
    //no operators left, just return value
    Ok(Node{
        left: None,
        right: None,
        value: Value::from(str.to_string())
    })
}
fn statement_string(node : &Node) -> String{
    if node.left.is_some() && node.right.is_some() {
        let mut total = statement_string(node.left.as_ref().unwrap().as_ref());
            total.push_str(node.value.org_str.as_str());
            total.push_str(statement_string(node.right.as_ref().unwrap().as_ref()).as_str());
        return total;
    }else{
        node.value.org_str.clone()
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input_str = String::new();
    stdin.read_line(&mut input_str);
    let statement_str = input_str.trim();
    let root_node = evaluate(statement_str).unwrap();
}