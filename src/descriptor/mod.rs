
/// 
/// 
pub trait Descriptive : Eq {
  fn describe(&self) -> String;
}

/// The metadata of a project.
/// 
/// While comparing two image objects, we need to check if
/// the two image objects lying under same project.
/// 
/// To check if two images are both included in same project, compare 
/// two project descriptor with implemented `describe` method.
/// 
/// # Example
/// 
/// ```rust
/// 
/// use vismatch_svc::descriptor::*; 
/// 
/// let pa = ProjectDescriptor::new("12345", "Some");
/// 
/// ```
#[derive(Eq, PartialEq)]
pub struct ProjectDescriptor {
  project_id: String,
  project_name: String,
}

impl ProjectDescriptor {
  pub fn new(id: &str, name: &str) -> Self {
    Self { project_id: id.to_owned(), project_name: name.to_owned() }
  }
}

impl Descriptive for ProjectDescriptor {
    fn describe(&self) -> String {
        self.project_name.to_owned()
    }
}