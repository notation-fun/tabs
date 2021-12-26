use notation_tab::prelude::*;

pub mod test;

pub fn main() {
    write_tab(&test::new_tab(), "tabs/test.ron");
}
