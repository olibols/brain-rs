use brain_rs::FunctionRegistry;
use brain_rs::function_registry::CallError;

fn main() {
    println!("NPC Brain Example");

    let mut registry = FunctionRegistry::new();

    registry.register(
        String::from("move"),
        |args: &[String]| -> Result<String, CallError> {
            if args.len() != 2 {
                return Err(CallError::InvalidArguments("Expected 2 arguments".to_string()));
            }
            let x: f32 = args[0].parse().map_err(|_| CallError::InvalidArguments("Invalid x coordinate".to_string()))?;
            let y: f32 = args[1].parse().map_err(|_| CallError::InvalidArguments("Invalid y coordinate".to_string()))?;
            Ok(format!("Moved to ({}, {})", x, y))
        },
        String::from("move(x: float, y: float) -> string"),
    );

    println!("{:?}", registry.call(String::from("move"), &["10".to_string(), "20".to_string()]));
    println!("{:?}", registry.call(String::from("move"), &["25".to_string(), "32".to_string()]));
    println!("{:?}", registry.call(String::from("move"), &["12".to_string(), "457".to_string()]));
}