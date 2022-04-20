#[derive(Debug, PartialEq)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(operand) => stack.push(*operand),
            CalculatorInput::Add            => {
                let operand2 = stack.pop()?;
                let operand1 = stack.pop()?;
                stack.push(operand1 + operand2);
            }
            CalculatorInput::Subtract       => {
                let operand2 = stack.pop()?;
                let operand1 = stack.pop()?;
                stack.push(operand1 - operand2);
            }
            CalculatorInput::Multiply       => {
                let operand2 = stack.pop()?;
                let operand1 = stack.pop()?;
                stack.push(operand1 * operand2);
            }
            CalculatorInput::Divide         => {
                let operand2 = stack.pop()?;
                let operand1 = stack.pop()?;
                stack.push(operand1 / operand2);
            }
        }
    }

    if stack.len() != 1 {
        return None;
    }
    Some(stack[0])
}
