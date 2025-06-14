//1. calculator
//series of steps
//some steps will be depend on previuos steps
//setp 1 steps multipy 2*3
//step 2 division 10/2
//steps 3 adding the result of steps 1 and step 2
// class-> calculator

use std::panic;
use std::result::*;

// data->
// num1,num2,
// operations-> addition,substraction,division,multipy,
struct Calculator {
    num1: f64,
    num2: f64,
    operator: String,
}
impl Calculator {
    fn new(num1: f64, num2: f64, operator: String) -> Self {
        Self {
            num1,
            num2,
            operator,
        }
    }
    fn addition(&self) -> f64 {
        self.num1 + self.num2
    }
    fn substraction(&self) -> f64 {
        self.num1 - self.num2
    }
    fn division(&self) -> Result<f64, std::io::Error> {
        if self.num2 < 0.0 {
            Err(std::io::ErrorKind::InvalidData.into())
        } else {
            Ok(self.num1 / self.num2)
        }
    }
    fn multiply(&self) -> f64 {
        self.num1 * self.num2
    }
    fn perform_operation(&self) -> f64 {
        let result = match self.operator.as_str() {
            "+" => Self::addition(&self),
            "-" => Self::substraction(&self),
            "*" => Self::multiply(&self),
            "/" => Self::division(&self).expect("not a valid input"),
            _ => panic!("not a valid operation"),
        };
        result
    }
}

fn main() {
    let input1 = Calculator::new(2.0, 3.0, "*".to_string());
    let input2 = Calculator::new(10.0, 2.0, "/".to_string());
    let result1 = input1.perform_operation();
    let result2 = input2.perform_operation();
    let final_result = Calculator::new(result1, result2, "+".to_string());
    println!(" the final result is {}", final_result.perform_operation());
}
//
// question:
/*Design a system that can execute a series of computation steps, where some steps depend on the results of previous steps.
Each step performs a basic arithmetic operation like multiplication, division, addition, etc.

Each step should:

Have a unique ID.

Specify an operation (e.g., multiply, divide, add).

Use either literals or references to results of previous steps as inputs.

Execute respecting dependency order. */

// class Diagram
/*
+-------------------+
|     StepInput     |<-------------------+
+-------------------+                    |
| +Literal(f64)     |                    |
| +Reference(String)|                    |
+-------------------+                    |
                                         |
+-------------------+     1         *    |
|       Step        |--------------------+
+-------------------+
| id: String        |
| operation: OpType |
| inputs: Vec<StepInput> |
+-------------------+

+-------------------+
|      OpType       |
+-------------------+
| Add               |
| Subtract          |
| Multiply          |
| Divide            |
+-------------------+

+-----------------------------+
|     StepExecutorEngine     |
+-----------------------------+
| steps: HashMap<String, Step>|
| results: HashMap<String, f64>|
+-----------------------------+
| +execute_all()             |
+-----------------------------+

 */

//correct solution
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum StepInput {
    Literal(f64),
    Reference(String),
}

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
struct Step {
    id: String,
    operation: OpType,
    inputs: Vec<StepInput>,
}

struct StepExecutorEngine {
    steps: HashMap<String, Step>,
    results: HashMap<String, f64>,
}

impl StepExecutorEngine {
    fn new(steps: Vec<Step>) -> Self {
        let step_map = steps
            .into_iter()
            .map(|step| (step.id.clone(), step))
            .collect();
        StepExecutorEngine {
            steps: step_map,
            results: HashMap::new(),
        }
    }

    fn resolve_dependencies(&self) -> Result<Vec<String>, String> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut indegree: HashMap<String, usize> = HashMap::new();

        for (id, step) in &self.steps {
            indegree.entry(id.clone()).or_insert(0);
            for input in &step.inputs {
                if let StepInput::Reference(dep_id) = input {
                    graph.entry(dep_id.clone()).or_default().push(id.clone());
                    *indegree.entry(id.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut queue: VecDeque<String> = indegree
            .iter()
            .filter_map(|(k, &v)| if v == 0 { Some(k.clone()) } else { None })
            .collect();

        let mut ordered = vec![];

        while let Some(id) = queue.pop_front() {
            ordered.push(id.clone());
            if let Some(neighbors) = graph.get(&id) {
                for neighbor in neighbors {
                    if let Some(e) = indegree.get_mut(neighbor) {
                        *e -= 1;
                        if *e == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if ordered.len() != self.steps.len() {
            return Err("Cycle detected in step dependencies".into());
        }

        Ok(ordered)
    }

    fn evaluate_step(&self, step: &Step) -> Result<f64, String> {
        let mut values = vec![];

        for input in &step.inputs {
            match input {
                StepInput::Literal(v) => values.push(*v),
                StepInput::Reference(ref_id) => {
                    if let Some(val) = self.results.get(ref_id) {
                        values.push(*val);
                    } else {
                        return Err(format!("Reference '{}' not yet resolved", ref_id));
                    }
                }
            }
        }

        if values.len() < 2 {
            return Err("At least two inputs required".into());
        }

        let result = match step.operation {
            OpType::Add => values.iter().copied().sum(),
            OpType::Subtract => values[0] - values[1],
            OpType::Multiply => values.iter().copied().product(),
            OpType::Divide => {
                if values[1] == 0.0 {
                    return Err("Division by zero".into());
                }
                values[0] / values[1]
            }
        };

        Ok(result)
    }

    fn execute_all(&mut self) -> Result<(), String> {
        let order = self.resolve_dependencies()?;
        for id in order {
            let step = self.steps.get(&id).ok_or("Missing step")?;
            let result = self.evaluate_step(step)?;
            self.results.insert(id.clone(), result);
        }
        Ok(())
    }

    fn print_results(&self) {
        for (id, val) in &self.results {
            println!("{} => {}", id, val);
        }
    }
}
