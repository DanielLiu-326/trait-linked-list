#![feature(ptr_metadata)]
#![feature(negative_impls)]
#![feature(auto_traits)]

mod linked_list;
mod pointers;
mod macros;

pub use pointers::*;
pub use linked_list::*;



use std::fmt::{Debug, Formatter};

#[test]
fn test() {
    pub struct Tag1();
    impl ListTag for Tag1{
        type Trait =dyn Debug;
    }

    pub struct Tag2();
    impl ListTag for Tag2{
        type Trait = dyn Debug;
    }

    pub struct Test {
        ext1: NodeExtraData<Tag1>,
        ext2: NodeExtraData<Tag2>,
        value: usize,
    }

    impl Debug for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{},", self.value)?;
            return Ok(());
        }
    }
    impl Test {
        pub fn new(value: usize) -> Self {
            Self {
                ext1: Default::default(),
                ext2: Default::default(),
                value,
            }
        }
    }

    impl AsRef<<Tag1 as ListTag>::Trait> for Test {
        fn as_ref(&self) -> &<Tag1 as ListTag>::Trait {
            self
        }
    }
    impl AsMut<<Tag1 as ListTag>::Trait> for Test {
        fn as_mut(&mut self) -> &mut <Tag1 as ListTag>::Trait {
            self
        }
    }
    impl Implemented<<Tag1 as ListTag>::Trait> for Test {}


    impl Node<Tag1> for Test {
        fn extra_data(&self) -> &NodeExtraData<Tag1> {
            &self.ext1
        }
    }

    impl Node<Tag2> for Test {
        fn extra_data(&self) -> &NodeExtraData<Tag2> {
            &self.ext2
        }
    }

    let mut test1 = Test::new(1);
    let mut test2 = Test::new(2);
    let mut test3 = Test::new(3);

    let test_list_1 = List::<Tag1>::new();
    let test_list_2 = List::<Tag2>::new();

    test_list_1.insert_back(NodePtr::new(&mut test3));
    test_list_1.insert_back(NodePtr::new(&mut test1));
    test_list_1.insert_back(NodePtr::new(&mut test2));
    test_list_2.insert_back(NodePtr::new(&mut test1));
    test_list_2.insert_back(NodePtr::new(&mut test2));

    <Test as NodeExt<Tag1>>::remove(&test2);

    let mut ptr = test_list_1.head_next();

    while !ptr.get_next().is_null() {
        println!("{:?}", ptr.as_ref());
        ptr = ptr.get_next();
    }

    println!("------------");

    let mut ptr = test_list_2.head_next();
    while !ptr.get_next().is_null() {
        println!("{:?}", ptr.as_mut());
        ptr = ptr.get_next();
    }

}
