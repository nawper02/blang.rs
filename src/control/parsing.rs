use crate::control::flow::{matrix_mode_flow, program_mode_flow, stack_mode_flow, variables_mode_flow};
use crate::data::context::{AppContext, AppMode};
use crate::utils::misc::LoopControl;

pub(crate) fn parse_quick_cmds(context: &mut AppContext) {
    match context.input_buffer.as_str() {
        "quit" => {
            context.should_quit = LoopControl::Break;
            context.input_buffer.clear()
        },
        "clear" => {
            context.stack.clear();
            context.input_buffer.clear()
        },
        "*" => {
            context.input_buffer.clear()
        },
        "/" => {
            context.input_buffer.clear()
        },
        "+" => {
            context.input_buffer.clear()
        },
        "-" => {
            context.input_buffer.clear()
        },
        "^" => {
            context.input_buffer.clear()
        },
        _ => {}
    }
}

pub(crate) fn parse_input(context: &mut AppContext) {
    // Parse input into usable data
    let buf: String = context.input_buffer.clone();
    match ParsedInput::create_from_buf(buf) {
        Ok(parsed) => {
            // Route parsed input into respective flow
            match context.current_mode {
                AppMode::Stack => stack_mode_flow(parsed, context),
                AppMode::Program => program_mode_flow(parsed, context),
                AppMode::Variables => variables_mode_flow(parsed, context),
                AppMode::Matrix => matrix_mode_flow(parsed, context),
            }
        }
        Err(error) => {
            // todo: handle error
        }
    }
}

pub(crate) enum InputType {
    FunctionCall {
        name: String,
        args: Vec<String>,
    },
    Value(ValueType),
}

pub(crate) enum ValueType {
    Number(f64),
    Array(Vec<Vec<f64>>),
}

pub(crate) struct ParsedInput {
    pub(crate) input_type: InputType,
}

impl ParsedInput {
    pub(crate) fn create_from_buf(buf: String) -> Result<ParsedInput, String> {
        // Check for function calls
        if buf.starts_with('.') {
            let mut parts = buf.split_whitespace();
            if let Some(name) = parts.next() {
                let args = parts.map(|s| s.to_string()).collect::<Vec<String>>();
                return Ok(ParsedInput {
                    input_type: InputType::FunctionCall {
                        name: name.trim_start_matches('.').to_string(),
                        args,
                    },
                });
            } else {
                return Err("Invalid function call syntax.".to_string());
            }
        }

        // Try to parse the input as a single number
        if let Ok(num) = buf.parse::<f64>() {
            return Ok(ParsedInput {
                input_type: InputType::Value(ValueType::Number(num)),
            });
        }

        // Try to parse the input as an array of numbers
        let rows: Vec<&str> = buf.split(';').collect();
        let mut is_valid = true;
        let mut parsed_array = Vec::new();

        for row in rows {
            let parts: Vec<&str> = row.split_whitespace().collect();
            if parts.iter().all(|part| part.parse::<f64>().is_ok()) {
                let numbers = parts.iter()
                    .filter_map(|part| part.parse::<f64>().ok())
                    .collect::<Vec<f64>>();
                parsed_array.push(numbers);
            } else {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            return Ok(ParsedInput {
                input_type: InputType::Value(ValueType::Array(parsed_array)),
            });
        } else {
            // Handle invalid input
        }

        // If input is not a number, an array of numbers, or a valid function call, return an error
        Err("Failed to parse input.".to_string())
    }
}
