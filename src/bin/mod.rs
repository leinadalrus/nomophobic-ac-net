mod arc_collection_processor;

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn test_processing_collection() {
        let test = false;

        match test {
            true => println!("True"),
            false => println!("False"),
        }
    }
}

fn main() -> std::result::Result<(), bool> {
    let testings: [bool; 1] = [true];
    for test in testings {
        if test != true {
            return Err(test);
        }
    }

    Ok(())
}