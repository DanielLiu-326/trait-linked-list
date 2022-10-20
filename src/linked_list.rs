use std::alloc::{alloc, Layout};
use std::cell::Cell;
use std::ops::{ DerefMut};

use crate::pointers::PtrMut;

/// 一个类型实现了Implemented<Trait>，就看作它实现了Trait，这里用TraitObject代替Trait
pub trait Implemented<Trait: ?Sized>: AsRef<Trait> + AsMut<Trait> {}

/// ListTag是Tag实现的类型，Trait代表该List每个元素都实现的Trait，Tag用于区分不同的List并且指明对应LinkedList的Trait
pub trait ListTag: 'static {
    type Trait: ?Sized;
}

///
/// 节点的前向/后向指针类型
///
pub type NodePtr<TAG> = PtrMut<dyn Node<TAG>>;

///
/// 一个类型作为节点所需要的指针信息
///
pub struct NodeExtraData<TAG: ListTag> {
    next: Cell<NodePtr<TAG>>,
    prev: Cell<NodePtr<TAG>>,
}

impl<TAG: ListTag> Default for NodeExtraData<TAG> {
    fn default() -> Self {
        Self {
            next: Cell::new(NodePtr::null()),
            prev: Cell::new(NodePtr::null())
        }
    }
}

impl<TAG: ListTag> NodeExtraData<TAG> {
    #[inline]
    pub fn get_next(&self) -> NodePtr<TAG> {
        self.next.get()
    }
    #[inline]
    pub fn set_next(&self, next: NodePtr<TAG>) {
        self.next.set(next)
    }
    #[inline]
    pub fn get_prev(&self) -> NodePtr<TAG> {
        self.prev.get()
    }
    #[inline]
    pub fn set_prev(&self, prev: NodePtr<TAG>) {
        self.prev.set(prev)
    }
}


///
/// 一个类型想要当作List的节点就必须实现Node
///
/// **泛型参数:**
///
/// - TAG:同一个节点可以在不同的List中，节点要存储多个List的指针信息，用0大小的类型当作Tag来区分他们
///

pub trait Node<TAG: ListTag>: Implemented<TAG::Trait> {
    /// 返回节点的附加信息
    fn extra_data(&self) -> &NodeExtraData<TAG>;
}

///
/// NodeExt是对Node的补充,增加了实用方法
///
pub trait NodeExt<TAG: ListTag> {
    fn remove(&self);
    fn as_trait(&self) -> &TAG::Trait;
    fn as_trait_mut(&mut self) -> &TAG::Trait;
    fn get_next(&self) -> NodePtr<TAG>;
    fn set_next(&self, next: NodePtr<TAG>);
    fn get_prev(&self) -> NodePtr<TAG>;
    fn set_prev(&self, prev: NodePtr<TAG>);
}

impl<TAG: ListTag, T: Node<TAG> + ?Sized> NodeExt<TAG> for T {
    #[inline]
    fn remove(&self) {
        self.get_prev().set_next(self.get_next());
        self.get_next().set_prev(self.get_prev());
    }
    #[inline]
    fn as_trait(&self) -> &TAG::Trait {
        return self.as_ref()
    }
    #[inline]
    fn as_trait_mut(&mut self) -> &TAG::Trait {
        return self.as_mut()
    }
    #[inline]
    fn get_next(&self) -> NodePtr<TAG> {
        self.extra_data().get_next()
    }
    #[inline]
    fn set_next(&self, next: NodePtr<TAG>) {
        self.extra_data().set_next(next)
    }
    #[inline]
    fn get_prev(&self) -> NodePtr<TAG> {
        self.extra_data().get_prev()
    }
    #[inline]
    fn set_prev(&self, prev: NodePtr<TAG>) {
        self.extra_data().set_prev(prev)
    }
}

///
/// 头节点实现。
///
struct HeadNode<TAG: ListTag> {
    extra: NodeExtraData<TAG>,
}

impl<TAG: ListTag> Implemented<TAG::Trait> for HeadNode<TAG> {}

impl<TAG: ListTag> AsRef<TAG::Trait> for HeadNode<TAG> {
    fn as_ref(&self) -> &TAG::Trait {
        panic!("called AsRef::as_ref() on head node")
    }
}

impl<TAG: ListTag> AsMut<TAG::Trait> for HeadNode<TAG> {
    fn as_mut(&mut self) -> &mut TAG::Trait {
        panic!("called AsMut::as_mut() on head node")
    }
}

impl<TAG: ListTag> Node<TAG> for HeadNode<TAG> {
    fn extra_data(&self) -> &NodeExtraData<TAG> {
        &self.extra
    }
}


pub struct List<TAG: ListTag> {
    head: NodePtr<TAG>,
    rear: NodePtr<TAG>,
}

impl<TAG: ListTag> Drop for List<TAG> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.head.thin().cast(),
                Layout::new::<[HeadNode<TAG>; 2]>(),
            );
        }
    }
}


impl<TAG: ListTag> List<TAG> {
    pub fn new() -> Self {
        unsafe {
            /*
             * 分配节点
             */
            let head_nodes = alloc(Layout::new::<[HeadNode<TAG>; 2]>())
                .cast::<[HeadNode<TAG>; 2]>();
            let head = NodePtr::new(&mut (*head_nodes)[0]);
            let rear = NodePtr::new(&mut (*head_nodes)[1]);
            /*
             * 连接头尾节点。
             */
            head.set_prev(NodePtr::null());
            head.set_next(rear);
            rear.set_prev(head);
            rear.set_next(NodePtr::null());
            Self {
                head,
                rear,
            }
        }
    }

    /// 返回头节点的下一个节点，如果链表为空返回尾节点
    pub fn head_next(&self) -> NodePtr<TAG> {
        self.head.get_next()
    }

    /// 头插节点
    pub fn insert_front<U: DerefMut<Target=dyn Node<TAG>>>(&self, mut node: U) {
        let head = self.head;
        let node = NodePtr::new(node.deref_mut());
        head.get_next().set_prev(node);
        head.set_next(node);
        node.set_next(head.get_next());
        node.set_prev(head);
    }

    ///尾插节点
    pub fn insert_back<U: DerefMut<Target=dyn Node<TAG>>>(&self, mut node: U) {
        let rear = self.rear;
        let node = NodePtr::new(node.deref_mut());
        rear.get_prev().set_next(node);
        rear.set_prev(node);
        node.set_next(rear);
        node.set_prev(rear.get_prev());
    }

    /// 将另一个链表的全部节点头插到本链表的前面，时间复杂度O(1)
    pub fn concat_front(&self, other: &Self) {
        //remove the nodes from other list
        let other_first = other.head.get_next();
        let other_last = other.rear.get_prev();
        other.head.set_next(other.rear);
        other.rear.set_prev(other.head);
        //concat those nodes to the front of self
        other_last.set_next(self.head.get_next());
        self.head.get_next().set_prev(other_last);

        other_first.set_prev(self.head);
        self.head.set_next(other_last);
    }

    /// 将另一个链表的全部节点尾插到本链表的前面，时间复杂度O(1)
    pub fn concat_back(&self, other: &mut Self) {
        //remove the nodes from other list
        let other_first = other.head.get_next();
        let other_last = other.rear.get_prev();
        other.head.set_next(other.rear);
        other.rear.set_prev(other.head);
        //concat those nodes to the rear of self
        other_first.set_prev(self.rear.get_prev());
        self.rear.get_prev().set_next(other_first);

        other_last.set_next(self.rear);
        self.rear.set_prev(other_last);
    }
    /// 链表是否为空
    pub fn empty(&self) -> bool {
        self.head.get_next().is_null()
    }
}