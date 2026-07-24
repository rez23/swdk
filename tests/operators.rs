use swdk::op::AsBuff;

struct Test(u32);

impl AsRef<Test> for Test {
    fn as_ref(&self) -> &Test {
        self
    }
}

impl AsBuff<Test> for Test {}

#[test]
fn as_buff_has_correct_size() {
    let value = Test(5);

    assert_eq!(
        value.as_buff().len(),
        core::mem::size_of::<Test>()
    );
}