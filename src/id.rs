use std::ops::Deref;

#[allow(dead_code)]
static mut COUNTER: u64 = 0;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID(pub(crate) u64);

impl ID {
    pub(crate) fn next() -> Self {
        unsafe {
            if COUNTER == u64::MAX {
                panic!("I don't know how but you ran out of ID's")
            }

            COUNTER += 1;
            Self(COUNTER - 1)
        }
    }
}

impl Deref for ID {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
