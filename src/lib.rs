#[stabby::stabby]
pub trait Snippet {}

#[stabby::stabby]
pub trait Segment {
	extern "C" fn display(&self) -> stabby::string::String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
