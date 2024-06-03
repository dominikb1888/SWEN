# Session 9

- Exercise Review

Rust:
- Recap Rust Memory Management: Sizing, Immutability


### Pointers:
https://doc.rust-lang.org/nightly/book/ch15-00-smart-pointers.html
Box vs. Rc vs. RefCell

- Box<T> is for single ownership
- Rc<T> is for multiple ownership
- RefCell<T>

- Arc<T> is for multiple ownership, but threadsafe.
- Cell<T> is for "interior mutability" for Copy types; that is, when you need to mutate something behind a &T.

- Generics
(Matzinger Book, pp.99-113)

Data Structures (Matztinger, pp.115-164):
- Recap Linked List
- Doubly Linked List
- Skip Lists and Heaps
- Dynamic Arrays (Vec<T>)

Algorithms:
- Quicksort
- Merge Sort
- Radix Sort
- Comparison and Benchmark



### Pointer Types (extended)

- Explicitly store on the heap: Box
- Shared ownership: Arc and Rc
- Mutate something shared: Cell, RefCell, Mutex

You use Box when you want to opt-in to storing something on the heap, which otherwise wouldn't go there. You should do this when:

- You are writing a recursive type.
- You want to use dyn SomeTrait, and you want ownership of it.
- You are actively optimizing code, and you have a really large struct that is expensive to move around.

You use Arc and Rc when you want to have shared ownership to some value. This means that you want to access the same variable from many different places, and any one of them should keep the value alive. The only distinction between Arc and Rc is that the former is very slightly more expensive, but the latter is not thread-safe. Anything shared with Rc/Arc is immutable.

You use Cell, RefCell or Mutex when you have something that is shared, and you want mutable access to it. The two cell types are not thread-safe, so if you are doing anything multi-threaded, you should use a mutex. As for choosing between Cell and RefCell, you should pick a Cell when the type is Copy, and RefCell otherwise.

So when you see Rc<RefCell<...>> or Arc<Mutex<...>>, this is because people have a value they want to share, but they also want to mutate it. The Rc/Arc allows them to share the value, and the RefCell/Mutex allows them to mutate it.

Some notes:

There are other ways to share than Arc/Rc. If you put a variable in a global, that's also sharing it, and in this case you need just the Mutex to mutate it. An Arc would not be necessary since the global already takes care of sharing it.

If you want to share a value, but don't need to mutate it, you can just use a Rc/Arc without a RefCell/Mutex.

Source: https://www.reddit.com/r/rust/comments/llzewm/when_should_i_use_box_arc_rc_cell_and_refcell_can/

