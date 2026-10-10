//          ╭─────────────────────────────────────────────────────────╮
//          │                  LIFETIMES in STRUCTS                   │
//          ╰─────────────────────────────────────────────────────────╯

// INFO: Struct that store references must declare lifetimes for those references.
// INFO: This ensure the struct does not outlive the referenced data.
struct ArrayProcessor<'a> {
    data: &'a [i32],
}

// INFO: The impl block for struct must include the generic lifetime used in the struct definition.
// INFO: This ties all method behavior to the struct's stored reference lifetime.
impl<'a> ArrayProcessor<'a> {
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
        let previous_data = self.data;
        self.data = new_data;
        previous_data
    }
}
fn main() {
    let mut some_data = ArrayProcessor { data: &[3, 4, 5] };
    let previous_data = some_data.update_data(&[1, 2, 3]);
    println!("Previous data: {:?}", previous_data);
    println!("New data: {:?}", some_data.data);
}
