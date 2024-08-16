use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{write, Formatter};

// Shared trait for all function calls
pub trait Callable: Send + Sync {
    fn call(&self, args: &[String]) -> Result<String, CallError>;
    fn signature(&self) -> String;
}

// New error type for parsed function calls
#[derive(Debug)]
pub enum CallError {
    InvalidArguments(String),
    ExecutionError(String),
}

// Display implementation for error messages
impl fmt::Display for CallError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CallError::InvalidArguments(msg) => { write!(f, "Invalid arguments {}", msg) }
            CallError::ExecutionError(msg) => { write!(f, "Execution error: {}", msg) }
        }
    }
}

// Wrapper for function with variable signatures
struct FunctionWrapper<FunctionType, ReturnType>
where
    FunctionType: Fn(&[String]) -> Result<ReturnType, CallError> + Send + Sync,
    ReturnType: ToString,
{
    func: FunctionType,
    signature: String,
}

impl<FunctionType, ReturnType> Callable for FunctionWrapper<FunctionType, ReturnType>
where
    FunctionType: Fn(&[String]) -> Result<ReturnType, CallError> + Send + Sync,
    ReturnType: ToString,
{
    fn call(&self, args: &[String]) -> Result<String, CallError> {
        (self.func)(args).map(|r| { r.to_string() })
    }

    fn signature(&self) -> String {
        self.signature.clone()
    }
}

pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn Callable>>
}

impl FunctionRegistry {
    pub fn new() -> Self {
        FunctionRegistry{ functions: HashMap::new() }
    }

    pub fn register<FunctionType, ReturnType>(&mut self, name: String, func: FunctionType, signature: String)
    where
        FunctionType: Fn(&[String]) -> Result<ReturnType, CallError> + Send + Sync + 'static,
        ReturnType: ToString + 'static,
    {
        let wrapper = FunctionWrapper {
            func: Box::new(func),
            signature,
        };
        self.functions.insert(name, Box::new(wrapper));
    }

    pub fn call(&mut self, name: String, args: &[String]) -> Result<String, CallError>
    {
        self.functions.get(&name).ok_or(CallError::InvalidArguments(format!("Function {} not found in registry", name)))?.call(args)
    }
}