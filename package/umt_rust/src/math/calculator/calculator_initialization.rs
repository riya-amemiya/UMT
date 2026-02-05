<<<<<<< HEAD
<<<<<<< HEAD
use super::calc::umt_calculator;
||||||| 30d5753
use super::calc::umt_calculator;
=======
use super::calculator::umt_calculator;
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247
use crate::object::Value;
use std::collections::HashMap;

/// Initializes a calculator function with exchange rates
///
/// # Arguments
///
/// * `exchange` - Exchange rates object
///
/// # Returns
///
/// Configured calculator closure
pub fn umt_calculator_initialization(exchange: HashMap<String, Value>) -> impl Fn(&str) -> String {
    move |x: &str| umt_calculator(x, Some(&exchange))
}
||||||| 55b8153
=======
use std::collections::HashMap;
use std::sync::Arc;

use super::entry::umt_calculator;

/// A configured calculator with pre-set exchange rates.
pub struct Calculator {
    exchange: HashMap<String, f64>,
}

impl Calculator {
    /// Creates a new Calculator with the given exchange rates.
    ///
    /// # Arguments
    ///
    /// * `exchange` - HashMap of currency symbols to exchange rates
    ///
    /// # Returns
    ///
    /// A new Calculator instance.
    pub fn new(exchange: HashMap<String, f64>) -> Self {
        Calculator { exchange }
    }

    /// Calculates the given expression using the configured exchange rates.
    ///
    /// # Arguments
    ///
    /// * `expression` - Mathematical expression string
    ///
    /// # Returns
    ///
    /// Calculation result as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use umt_rust::math::calculator::Calculator;
    /// use std::collections::HashMap;
    ///
    /// let mut rates = HashMap::new();
    /// rates.insert("$".to_string(), 100.0);
    /// let calc = Calculator::new(rates);
    /// assert_eq!(calc.calculate("$1"), "100");
    /// ```
    pub fn calculate(&self, expression: &str) -> String {
        umt_calculator(expression, Some(&self.exchange))
    }
}

/// Initializes a calculator function with exchange rates.
///
/// # Arguments
///
/// * `exchange` - HashMap of currency symbols to exchange rates
///
/// # Returns
///
/// A Calculator struct that can be used to calculate expressions.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_calculator_initialization;
/// use std::collections::HashMap;
///
/// let mut rates = HashMap::new();
/// rates.insert("$".to_string(), 100.0);
/// let calc = umt_calculator_initialization(rates);
/// assert_eq!(calc.calculate("$1"), "100");
/// ```
pub fn umt_calculator_initialization(exchange: HashMap<String, f64>) -> Calculator {
    Calculator::new(exchange)
}

/// Creates a calculator closure with pre-configured exchange rates.
///
/// # Arguments
///
/// * `exchange` - HashMap of currency symbols to exchange rates
///
/// # Returns
///
/// A boxed closure that calculates expressions.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_calculator_initialization_fn;
/// use std::collections::HashMap;
///
/// let mut rates = HashMap::new();
/// rates.insert("EUR".to_string(), 1.2);
/// let calc = umt_calculator_initialization_fn(rates);
/// assert_eq!(calc("EUR5+10"), "16");
/// ```
pub fn umt_calculator_initialization_fn(
    exchange: HashMap<String, f64>,
) -> Box<dyn Fn(&str) -> String> {
    let exchange = Arc::new(exchange);
    Box::new(move |x: &str| umt_calculator(x, Some(&exchange)))
}
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
