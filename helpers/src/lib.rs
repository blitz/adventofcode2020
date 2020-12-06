use std::io::{self, BufRead};

pub fn to_lines<T: BufRead>(input: &mut T) -> io::Result<Vec<String>>
{
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
