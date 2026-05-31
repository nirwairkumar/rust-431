// Error Handling techniques [2 approches]

fn main() {
    // Approach 1
    enum Option<T>{  // Define the generic Option type
        Some(T), // Represents a value
        None,    // Represents no value
    }

    // Approach 2
    enum Result<T,E>{  // Define the generic Result Type
        Ok(T), // Represents a value
        Err(E), // Represents an error
    }   // 

}
