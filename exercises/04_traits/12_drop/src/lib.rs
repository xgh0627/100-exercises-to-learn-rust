// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

struct DropBomb;

impl DropBomb {
    fn new() -> Self {
        Self
    }

    fn defuse(&self){

    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
      panic!("some panic")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

}
