use std::fmt;

pub struct NoSuchModelError(&str);

impl fmt::Display for NoSuchModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let NoSuchModelError(provided_model_name) = self;
        write!(f, "No model named {} found.", provided_model_name)
    }
}
