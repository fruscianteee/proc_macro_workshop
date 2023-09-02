// Generate a `build` method to go from builder to original struct.
//
// This method should require that every one of the fields has been explicitly set; it should return an error if a field is missing. The precise error type is not important.
// Consider using Box<dyn Error>, which you can construct using the impl From<String> for Box<dyn Error>.
//
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
//     }

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

//pub struct CommandBuilder {
//    executable: Option<String>,
//    args: Option<Vec<String>>,
//    env: Option<Vec<String>>,
//    current_dir: Option<String>,
//}
//
//impl CommandBuilder {
//    fn executable(&mut self, executable: String) -> &mut Self {
//        self.executable = Some(executable);
//        self
//    }
//    fn args(&mut self, args: Vec<String>) -> &mut Self {
//        self.args = Some(args);
//        self
//    }
//    fn env(&mut self, env: Vec<String>) -> &mut Self {
//        self.env = Some(env);
//        self
//    }
//    fn current_dir(&mut self, current_dir: String) -> &mut Self {
//        self.current_dir = Some(current_dir);
//        self
//    }
//
//    fn build(&self) -> Result<Command, Box<dyn Error>> {
//        let command = Command {
//            executable: self.executable.ok_or("not found executable".into())?,
//            args: self.args.ok_or("not found args".into())?,
//            env: self.env.ok_or("not found env".into())?,
//            current_dir: self.current_dir.ok_or("not found current_dir".into())?,
//        };
//        Ok(command)
//    }
//}

#[test]
fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    builder.env(vec![]);
    builder.current_dir("..".to_owned());

    let command = builder.build().unwrap();
    assert_eq!(command.executable, "cargo");
}
