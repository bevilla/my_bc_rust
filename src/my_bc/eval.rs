use crate::my_bc::{ Number, BASE_10, from_base, clone, negate, add, sub, mul, div, modulo };

#[derive(Debug, Copy, Clone)]
enum UnaryOperator {
    Minus,
}

#[derive(Debug, Copy, Clone)]
enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    BeginGroup,
    EndGroup,
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

#[derive(Debug)]
enum ExpressionNodeData {
    Operand(Number),
    Operator(Operator),
}

#[derive(Debug)]
enum ExpressionNode {
    Node(ExpressionNodeData, Box<ExpressionNode>),
    Nil,
}

#[derive(Debug)]
pub struct Infix(ExpressionNode);
#[derive(Debug)]
pub struct Postfix(ExpressionNode);

enum State {
    ExpectOperand,
    ExpectOperator,
}

pub fn parse_infix(s: &str, base: &str) -> Result<Infix, String> {
    fn aux(s: &str, base: &str, index:usize, state: State, node: ExpressionNode, group_count: i32) -> Result<Infix, String> {
        if let Some(c) = s.chars().nth(index) {
            match c {
                '(' => {
                    debug_assert!(group_count >= 0);
                    aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::BeginGroup), Box::new(node)), group_count + 1)
                },
                ')' => {
                    if group_count <= 0 {
                        Err(format!("{}: mismatched parenthesis", index))
                    } else {
                        aux(s, base, index + 1, State::ExpectOperator, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::EndGroup), Box::new(node)), group_count - 1)
                    }
                },
                _ =>
                    match state {
                        State::ExpectOperand => {
                            match c {
                                '-' => {
                                    aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Unary(UnaryOperator::Minus)), Box::new(node)), group_count)
                                },
                                '+' => {
                                    aux(s, base, index + 1, State::ExpectOperand, node, group_count)
                                },
                                _ if base.find(c).is_some() => {
                                    let (number, new_index) = {
                                        if let Some(new_index) = s[index..].find(|e| base.find(e).is_none()) {
                                            let new_index = index + new_index;
                                            (&s[index..new_index], new_index)
                                        } else {
                                            (&s[index..], s.len())
                                        }
                                    };
                                    let number = from_base(number, base).expect("unexpected error");
                                    aux(s, base, new_index, State::ExpectOperator, ExpressionNode::Node(ExpressionNodeData::Operand(number), Box::new(node)), group_count)
                                },
                                _ => {
                                    Err(format!("{}: invalid character '{}'", index, c))
                                },
                            }
                        },
                        State::ExpectOperator =>
                            match c {
                                '+' => aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Binary(BinaryOperator::Add)), Box::new(node)), group_count),
                                '-' => aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Binary(BinaryOperator::Sub)), Box::new(node)), group_count),
                                '*' => aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Binary(BinaryOperator::Mul)), Box::new(node)), group_count),
                                '/' => aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Binary(BinaryOperator::Div)), Box::new(node)), group_count),
                                '%' => aux(s, base, index + 1, State::ExpectOperand, ExpressionNode::Node(ExpressionNodeData::Operator(Operator::Binary(BinaryOperator::Modulo)), Box::new(node)), group_count),
                                _ => Err(format!("{}: invalid character '{}'", index, c)),
                            },
                    }
            }
        } else {
            if group_count == 0 {
                Ok(Infix(node))
            } else {
                Err(format!("{}: mismatched parenthesis", index))
            }
        }
    }
    aux(s, base, 0, State::ExpectOperand, ExpressionNode::Nil, 0)
}

fn is_expression_node_data_begin_group(data: &ExpressionNodeData) -> bool {
    match data {
        ExpressionNodeData::Operand(_) => false,
        ExpressionNodeData::Operator(operator) => match operator {
            Operator::BeginGroup => true,
            _ => false,
        },
    }
}

fn get_expression_node_data_precedence(data: &ExpressionNodeData) -> i32 {
    match data {
        ExpressionNodeData::Operand(_) => 0,
        ExpressionNodeData::Operator(operator) => match operator {
            Operator::BeginGroup => 0,
            Operator::EndGroup => 0,
            Operator::Binary(BinaryOperator::Add) => 1,
            Operator::Binary(BinaryOperator::Sub) => 1,
            Operator::Binary(BinaryOperator::Mul) => 2,
            Operator::Binary(BinaryOperator::Div) => 2,
            Operator::Binary(BinaryOperator::Modulo) => 2,
            Operator::Unary(UnaryOperator::Minus) => 3,
        },
    }
} 

pub fn infix_to_postfix(node: &Infix) -> Postfix {
    fn push_operand_to_infix(operand: Number, infix: ExpressionNode, stack: ExpressionNode) -> (ExpressionNode, ExpressionNode) {
        let infix = ExpressionNode::Node(ExpressionNodeData::Operand(operand), Box::new(infix));
        (infix, stack)
    }
    fn push_operator_to_stack(operator: &Operator, infix: ExpressionNode, stack: ExpressionNode) -> (ExpressionNode, ExpressionNode) {
        let stack = ExpressionNode::Node(ExpressionNodeData::Operator(*operator), Box::new(stack));
        (infix, stack)
    }
    fn push_stack_to_infix(infix: ExpressionNode, stack: ExpressionNode, stop_if: &dyn Fn(&ExpressionNodeData) -> bool) -> (ExpressionNode, ExpressionNode) {
        match stack {
            ExpressionNode::Node(ref data, _) => 
                if stop_if(data) {
                    (infix, stack)
                } else {
                    if let ExpressionNode::Node(a, b) = stack {
                        push_stack_to_infix(ExpressionNode::Node(a, Box::new(infix)), *b, stop_if)
                    } else {
                        panic!("error")
                    }
                },
            ExpressionNode::Nil => (infix, stack),
        }
    }
    fn to_infix_and_stack(node: &ExpressionNode, infix: ExpressionNode, stack: ExpressionNode) -> (ExpressionNode, ExpressionNode) {
        match node {
            ExpressionNode::Node(data, next) => {
                let (infix, stack) = to_infix_and_stack(next, infix, stack);

                match data {
                    ExpressionNodeData::Operand(operand) => push_operand_to_infix(clone(operand), infix, stack),
                    ExpressionNodeData::Operator(operator) => {
                        match operator {
                            Operator::BeginGroup => push_operator_to_stack(operator, infix, stack),
                            Operator::EndGroup => {
                                let (infix, stack) = push_stack_to_infix(infix, stack, &is_expression_node_data_begin_group);
                                if let ExpressionNode::Node(_, next) = stack { // pop '('
                                    (infix, *next)
                                } else {
                                    (infix, stack)
                                }
                            },
                            Operator::Unary(_) => {
                                let (infix, stack) = push_stack_to_infix(infix, stack, &|stack_data| get_expression_node_data_precedence(&data) >= get_expression_node_data_precedence(stack_data));
                                push_operator_to_stack(operator, infix, stack)
                            },
                            Operator::Binary(_) => {
                                let (infix, stack) = push_stack_to_infix(infix, stack, &|stack_data| get_expression_node_data_precedence(&data) > get_expression_node_data_precedence(stack_data));
                                push_operator_to_stack(operator, infix, stack)
                            },
                        }
                    }
                }
            },
            ExpressionNode::Nil => (infix, stack),
        }
    }
    fn reverse(node: ExpressionNode) -> ExpressionNode {
        fn aux(node: ExpressionNode, reversed: ExpressionNode) -> ExpressionNode {
            match node {
                ExpressionNode::Node(data, next) => aux(*next, ExpressionNode::Node(data, Box::new(reversed))),
                ExpressionNode::Nil => reversed,
            }
        }
        aux(node, ExpressionNode::Nil)
    }
    let (infix, stack) = to_infix_and_stack(&node.0, ExpressionNode::Nil, ExpressionNode::Nil);
    let (infix, _) = push_stack_to_infix(infix, stack, &|_| false);
    let infix = reverse(infix);
    Postfix(infix)
}

pub fn eval_postfix(node: &Postfix) -> Result<Number, String> {
    fn do_op1(stack: ExpressionNode, f: fn(&Number) -> Number) -> ExpressionNode {
        if let ExpressionNode::Node(ExpressionNodeData::Operand(number), stack) = stack {
            ExpressionNode::Node(ExpressionNodeData::Operand(f(&number)), stack)
        } else {
            panic!("unexpected error")
        }
    }
    fn do_op2(stack: ExpressionNode, f: fn(&Number, &Number) -> Number) -> ExpressionNode {
        if let ExpressionNode::Node(ExpressionNodeData::Operand(rhs), stack) = stack {
            if let ExpressionNode::Node(ExpressionNodeData::Operand(lhs), stack) = *stack {
                let result = f(&lhs, &rhs);
                ExpressionNode::Node(ExpressionNodeData::Operand(result), stack)
            } else {
                panic!("unexpected error")
            }
        } else {
            panic!("unexpected error")
        }
    }
    fn do_op2_can_fail(stack: ExpressionNode, f: fn(&Number, &Number) -> Result<Number, String>) -> Result<ExpressionNode, String> {
        if let ExpressionNode::Node(ExpressionNodeData::Operand(rhs), stack) = stack {
            if let ExpressionNode::Node(ExpressionNodeData::Operand(lhs), stack) = *stack {
                match f(&lhs, &rhs) {
                    Ok(result) => Ok(ExpressionNode::Node(ExpressionNodeData::Operand(result), stack)),
                    Err(err) => Err(err),
                }
            } else {
                panic!("unexpected error")
            }
        } else {
            panic!("unexpected error")
        }
    }
    fn do_op(stack: ExpressionNode, op: &Operator) -> Result<ExpressionNode, String> {
        match op {
            Operator::BeginGroup => panic!("unexpected error"),
            Operator::EndGroup => panic!("unexpected error"),
            Operator::Unary(op) => {
                match op {
                    UnaryOperator::Minus => Ok(do_op1(stack, negate)),
                }
            },
            Operator::Binary(op) => {
                match op {
                    BinaryOperator::Add => Ok(do_op2(stack, add)),
                    BinaryOperator::Sub => Ok(do_op2(stack, sub)),
                    BinaryOperator::Mul => Ok(do_op2(stack, mul)),
                    BinaryOperator::Div => do_op2_can_fail(stack, div),
                    BinaryOperator::Modulo => do_op2_can_fail(stack, modulo),
                }
            },
        }
    }
    fn aux(postfix: &ExpressionNode, stack: ExpressionNode) -> Result<Number, String> {
        match postfix {
            ExpressionNode::Node(data, next) => {
                match data {
                    ExpressionNodeData::Operand(number) => aux(next, ExpressionNode::Node(ExpressionNodeData::Operand(clone(number)), Box::new(stack))),
                    ExpressionNodeData::Operator(op) => aux(next, do_op(stack, op)?)
                }
            },
            ExpressionNode::Nil => {
                if let ExpressionNode::Node(ExpressionNodeData::Operand(n), _) = stack {
                    Ok(n)
                } else {
                    panic!("unexpected error")
                }
            }
        }
    }
    aux(&node.0, ExpressionNode::Nil)
}

pub fn eval_base(expression: &str, base: &str) -> Result<Number, String> {
    match parse_infix(expression, base) {
        Ok(infix) => {
            eval_postfix(&infix_to_postfix(&infix))
        },
        Err(err) => {
            Err(err)
        },
    }
}

pub fn eval(expression: &str) -> Result<Number, String> {
    eval_base(expression, BASE_10)
}
