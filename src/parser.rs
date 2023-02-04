use std::{path::Path, collections::{HashMap, HashSet}};

/// The parser structure has the purpose of reading a .env file and setting
/// environment files based on what it reads from .env
/// env_vars: Is a hashmap used to read and set env variables
/// set_variables: Set of variables that it managed to set.
#[allow(dead_code)]
pub struct Parser {
    env_vars: HashMap<String, String>,
    pub set_variables: HashSet<String>,
}

impl Parser {

    #[allow(dead_code)]
    pub fn new () -> Parser {
        Parser {
            env_vars: HashMap::new(),
            set_variables: HashSet::new()
        }
     }

     /// Reads variables in a .env file provided as a parameter.
     /// Sets environment variables based on the variables it reads from the file.
     /// It will ignore any line in the file that does not respect the format: Key=Value
     /// If the provided path is not a .env file it will default to ".env"
     /// May throw std::io:Error if it fails reading from the file for any reason
     /// or panic if it fails to set an environment variable.
     /// 
     /// It builds a vector of names on the self object, representing the variables
     /// that it managed to set.
     #[allow(dead_code)]
     pub fn parse(&mut self, path: &Path) -> Result<(), std::io::Error> {

        let content = Parser::load_dotenv_file(path)?;
        let content: Vec<&str> = content.split('\n').collect();

        for line in content {
            let mut key_value_split = line.split('=');
            
            // A variable should have a key and a value separated by the '=' sign.
            let key = match key_value_split.next() {
                Some(key) => key,
                None => continue,
            };

            let value = match key_value_split.next() {
                Some(value) => value,
                None => continue,
            };

            self.env_vars.insert(key.to_string(), value.to_string());
        }

        for key in self.env_vars.keys() {
            std::env::set_var(key, &self.env_vars[key]);
            self.set_variables.insert(key.to_string());
        }
        
        Ok(())

     }

     fn load_dotenv_file(path: &Path) -> Result<String, std::io::Error> {
        let mut path = path;

        if !path.ends_with(".env") { 
            println!("The provided path is not a .env file... Defaulting to .env");
            path = Path::new(".env"); 
        }

        let content = std::fs::read_to_string(path)?;
        return Ok(content);
     }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use std::path::Path;

    #[test]
    fn load_dotenv_file() {
        let mut parser = Parser::new();
        let res = parser.parse(Path::new(".env"));

        assert!(res.is_ok())

    }

    #[test]
    fn set_variables() {
        let mut parser = Parser::new();
        parser.parse(Path::new(".env"));

        assert!(parser.set_variables.len() == 1);
        assert!(parser.set_variables.contains("GMAPS_API_KEY"));
    }

}