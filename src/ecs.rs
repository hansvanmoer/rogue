use std::alloc::{Layout, LayoutError};
use std::collections::BinaryHeap;
use std::ptr::NonNull;

pub type EntityId = usize;

///
/// Marks a type as a component
///
pub trait Component : 'static {
    ///
    /// The unique component name
    ///
    fn get_component_name() -> &'static str;
}

pub struct World {
    ///
    /// The columns
    ///
    columns: Vec<Column>,

    ///
    /// The data array
    ///
    data: NonNull<u8>,

    ///
    /// The capacity
    ///
    capacity: usize,

    ///
    /// The number of entities
    ///
    amount: usize,

    ///
    /// The layout of the header
    ///
    header_layout: Layout,

    ///
    /// The layout of a single entity
    ///
    layout: Layout,

    ///
    /// The free list
    ///
    free_list: BinaryHeap<usize>,
}

impl World {

    ///
    /// Sets the capacity of the world
    ///
    pub fn set_capacity(&mut self, capacity: usize) -> Result<(), Error> {
        let layout = Layout::from_size_align(capacity * self.layout.size(), self.layout.align())?;
        if capacity > self.amount {
            if self.capacity == 0 {
                let data = unsafe {
                    NonNull::new(std::alloc::alloc(layout)).ok_or(Error::MemoryAllocationFailed)
                }?;
                self.data = data;
                self.capacity = capacity;
            } else {
                let old_layout = Layout::from_size_align(self.capacity * self.layout.size(), self.layout.align())?;
                let data = unsafe {
                    NonNull::new(std::alloc::realloc(self.data.as_ptr(), old_layout, layout.size())).ok_or(Error::MemoryAllocationFailed)
                }?;
                self.data = data;
                self.capacity = capacity;
            }
        }
        Ok(())
    }

    ///
    /// The next free ID
    ///
    fn next_free_id(&mut self) -> usize {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            self.amount
        }
    }

    ///
    /// Inserts a new entity with a single component
    ///
    pub fn insert1<C0: Component>(&mut self, c0: C0) -> Result<EntityId, Error> {
        let column0 = self.find_column::<C0>()?;
        self.reserve_capacity()?;
        let index = self.next_free_id();
        self.set_mask(index, self.columns[column0].mask);
        self.set_column(index, column0, c0);
        self.amount += 1;
        Ok(index)
    }

    ///
    /// Inserts a new entity with a single component
    ///
    pub fn insert2<C0: Component, C1: Component>(&mut self, c0: C0, c1: C1) -> Result<EntityId, Error> {
        let column0 = self.find_column::<C0>()?;
        let column1 = self.find_column::<C1>()?;
        self.reserve_capacity()?;
        let index = self.next_free_id();
        self.set_mask(index, self.columns[column0].mask | self.columns[column1].mask);
        self.set_column(index, column0, c0);
        self.set_column(index, column1, c1);
        self.amount += 1;
        Ok(index)
    }

    ///
    /// Queries on a single component
    ///
    pub fn query1<'a, C0: Component, F: Fn(&C0) -> bool>(&'a self, filter: F) -> Result<Query1<'a, C0, F>, Error> {
        Ok(Query1 {
            world: self,
            column0: self.find_column::<C0>()?,
            filter,
            phantom_data: std::marker::PhantomData,
        })
    }

    ///
    /// Queries on a single mutable component
    ///
    pub fn mut_query1<'a, C0: Component, F: Fn(&C0) -> bool>(&'a mut self, filter: F) -> Result<MutQuery1<'a, C0, F>, Error> {
        let column0 = self.find_column::<C0>()?;
        Ok(MutQuery1 {
            world: self,
            column0,
            filter,
            phantom_data0: std::marker::PhantomData,
        })
    }

    ///
    /// Removes an entity and all its components
    ///
    pub fn remove(&mut self, id: EntityId) -> Result<(), Error> {
        if id < self.capacity {
            let mask = self.get_mask(id);
            if mask == 0 {
                Err(Error::NoEntity)
            } else {
                self.drop_element(id);
                self.set_mask(id, 0);
                self.free_list.push(id);
                Ok(())
            }
        } else {
            Err(Error::NoEntity)
        }
    }

    ///
    /// Clears the entire world
    ///
    pub fn clear(&mut self) {
        for i in 0..self.amount {
            self.drop_element(i);
            self.set_mask(i, 0);
        }
    }

    fn drop_element(&mut self, index: usize) {
        let mask = self.get_mask(index);
        for column in &self.columns {
            if mask & column.mask == column.mask {
                let ptr = unsafe {
                    self.data.as_ptr().add(index * self.layout.size() + column.offset)
                };
                (column.drop_handle)(ptr);
            }
        }
    }

    fn get_mask(&self, index: usize) -> u32 {
        unsafe {
            *(self.data.as_ptr().add(index * self.layout.size()) as * const u32).as_ref().unwrap_or(&0)
        }
    }

    fn set_mask(&mut self, index: usize, mask: u32) {
        unsafe {
            if let Some(m) = (self.data.add(index * self.layout.size()).as_ptr() as * mut u32).as_mut() {
                *m = mask;
            }
        }
    }

    fn set_column<C>(&mut self, index: usize, column_index: usize, component: C) {
        unsafe {
            std::ptr::write(self.data.add(index * self.layout.size()).add(self.columns[column_index].offset).as_ptr() as *mut C, component);
        }
    }

    ///
    /// Reserves capacity for another item
    ///
    fn reserve_capacity(&mut self) -> Result<(), Error>{
        if self.amount == self.capacity {
            if self.capacity == 0 {
                self.set_capacity(1).map_err(|_| Error::MemoryAllocationFailed)
            } else {
                self.set_capacity(self.capacity * 2).map_err(|_| Error::MemoryAllocationFailed)
            }
        } else {
            Ok(())
        }
    }

    ///
    /// Finds a column by name
    ///
    fn find_column<C: Component>(&self) -> Result<usize, Error> {
        let name = C::get_component_name();
        for i in 0..self.columns.len() {
            if self.columns[i].name.eq(name) {
                return Ok(i);
            }
        }
        Err(Error::UnregisteredComponent)
    }
}

impl Drop for World {
    fn drop(&mut self) {
        for i in 0..self.amount {
            self.drop_element(i);
        }
        if self.capacity != 0 {
            unsafe {
                std::alloc::dealloc(self.data.as_ptr(), self.layout)
            }
        }
    }
}

///
/// A query
///
trait Query<'a> {

    type Item;

    fn get(&self, id: EntityId) -> Option<Self::Item>;

    fn max_amount(&self) -> usize;
}


struct Iter<'a, Q: Query<'a>> {
    query: Q,
    id: EntityId,
    phantom_data: std::marker::PhantomData<&'a Q::Item>,
}

impl<'a, Q: Query<'a>> Iter<'a, Q> {
    fn new (query: Q) -> Self {
        Self {
            query,
            id: 0,
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, Q: Query<'a>> Iterator for Iter<'a, Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.id == self.query.max_amount() {
                break None;
            } else {
                let result = self.query.get(self.id);
                self.id += 1;
                if result.is_some() {
                    break result;
                }
            }
        }
    }
}

///
/// A filtered query component
///
pub struct Query1<'a, C0: Component, F: Fn(&C0) -> bool> {
    ///
    /// The world
    ///
    world: &'a World,

    ///
    /// The index of the first column
    ///
    column0: usize,

    ///
    /// The filter applied to the query
    ///
    filter: F,

    ///
    /// A marker for the component type
    ///
    phantom_data: std::marker::PhantomData<C0>,
}

impl<'a, C0: Component, F: Fn(&C0) -> bool> Query<'a> for Query1<'a, C0, F> {

    ///
    /// The query result item
    ///
    type Item = &'a C0;

    ///
    /// Gets the query result for a given entity
    ///
    fn get(&self, id: EntityId) -> Option<Self::Item> {
        let mask = self.world.columns[self.column0].mask;
        unsafe {
            let flags = std::ptr::read(self.world.data.as_ptr().add(id * self.world.layout.size()) as * const u32);
            if flags & mask == mask {
                let ptr = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column0].offset) as * const C0;
                ptr.as_ref().filter(|c0| (self.filter)(*c0))
            } else {
                None
            }
        }
    }

    ///
    /// The maximum amount of entities that can be queried
    ///
    fn max_amount(&self) -> usize {
        self.world.amount
    }
}

impl<'a, C0: Component, F: Fn(&C0) -> bool> Query1<'a, C0, F> {

    ///
    /// turns the query into an iterator over its results
    ///
    pub fn into_iter(self) -> Iter<'a, Self> {
        Iter::new(self)
    }

}

///
/// A filtered query component
///
pub struct Query2<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> {
    ///
    /// The world
    ///
    world: &'a World,

    ///
    /// The index of the first column
    ///
    column0: usize,

    ///
    /// The index of the second column
    ///
    column1: usize,

    ///
    /// The filter applied to the query
    ///
    filter: F,

    ///
    /// A marker for the component type
    ///
    phantom_data: std::marker::PhantomData<(C0, C1)>,
}

impl<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> Query<'a> for Query2<'a, C0, C1, F> {

    ///
    /// The query result item
    ///
    type Item = (&'a C0, &'a C1);

    ///
    /// Gets the query result for a given entity
    ///
    fn get(&self, id: EntityId) -> Option<Self::Item> {
        let mask = self.world.columns[self.column0].mask;
        unsafe {
            let flags = std::ptr::read(self.world.data.as_ptr().add(id * self.world.layout.size()) as * const u32);
            if flags & mask == mask {
                let ptr0 = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column0].offset) as * const C0;
                let ptr1 = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column1].offset) as * const C1;
                match ptr0.as_ref() {
                    Some(c0) => {
                        match ptr1.as_ref() {
                            Some(c1 ) => {
                                if (self.filter)(c0, c1) {
                                    Some((c0, c1))
                                } else {
                                    None
                                }
                            },
                            None => None,
                        }
                    },
                    None => None,
                }
            } else {
                None
            }
        }
    }

    ///
    /// The maximum amount of entities that can be queried
    ///
    fn max_amount(&self) -> usize {
        self.world.amount
    }
}

impl<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> Query2<'a, C0, C1, F> {

    ///
    /// turns the query into an iterator over its results
    ///
    pub fn into_iter(self) -> Iter<'a, Self> {
        Iter::new(self)
    }

}

///
/// A query
///
trait MutQuery<'a> {

    type Item;

    fn get_mut(&mut self, id: EntityId) -> Option<Self::Item>;

    fn max_amount(&self) -> usize;
}

struct MutIter<'a, Q: MutQuery<'a>> {
    query: Q,
    id: EntityId,
    phantom_data: std::marker::PhantomData<&'a Q::Item>,
}

impl<'a, Q: MutQuery<'a>> MutIter<'a, Q> {
    fn new (query: Q) -> Self {
        Self {
            query,
            id: 0,
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, Q: MutQuery<'a>> Iterator for MutIter<'a, Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.id == self.query.max_amount() {
                break None;
            } else {
                let result = self.query.get_mut(self.id);
                self.id += 1;
                if result.is_some() {
                    break result;
                }
            }
        }
    }
}

pub struct MutQuery1<'a, C0: Component, F: Fn(&C0) -> bool> {
    world: &'a mut World,
    column0: usize,
    filter: F,
    phantom_data0: std::marker::PhantomData<C0>,
}

impl<'a, C0: Component, F: Fn(&C0) -> bool> MutQuery<'a> for MutQuery1<'a, C0, F> {
    type Item = &'a mut C0;

    fn get_mut(&mut self, id: EntityId) -> Option<Self::Item> {
        let mask = self.world.columns[self.column0].mask;
        unsafe {
            let flags = std::ptr::read(self.world.data.as_ptr().add(id * self.world.layout.size()) as * const u32);
            if flags & mask == mask {
                let ptr = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column0].offset) as * const C0;
                if ptr.as_ref().filter(|c0| (self.filter)(*c0)).is_some() {
                    ptr.cast_mut().as_mut()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn max_amount(&self) -> usize {
        self.world.amount
    }
}

impl<'a, C0: Component, F: Fn(&C0) -> bool> MutQuery1<'a, C0, F> {
    pub fn into_iter(self) -> MutIter<'a, Self> {
        MutIter::new(self)
    }
}

pub struct MutQuery2<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> {
    world: &'a mut World,
    column0: usize,
    column1: usize,
    filter: F,
    phantom_data0: std::marker::PhantomData<(C0, C1)>,
}

impl<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> MutQuery<'a> for MutQuery2<'a, C0, C1, F> {
    type Item = (&'a mut C0, &'a mut C1);

    fn get_mut(&mut self, id: EntityId) -> Option<Self::Item> {
        let mask = self.world.columns[self.column0].mask;
        unsafe {
            let flags = std::ptr::read(self.world.data.as_ptr().add(id * self.world.layout.size()) as * const u32);
            if flags & mask == mask {
                let ptr0 = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column0].offset) as * const C0;
                let ptr1 = self.world.data.as_ptr().add(id * self.world.layout.size() + self.world.columns[self.column1].offset) as * const C1;
                match ptr0.cast_mut().as_mut() {
                    Some(c0) => {
                        match ptr1.cast_mut().as_mut() {
                            Some(c1 ) => {
                                if (self.filter)(c0, c1) {
                                    Some((c0, c1))
                                } else {
                                    None
                                }
                            },
                            None => None,
                        }
                    },
                    None => None,
                }
            } else {
                None
            }
        }
    }

    fn max_amount(&self) -> usize {
        self.world.amount
    }
}

impl<'a, C0: Component, C1: Component, F: Fn(&C0, &C1) -> bool> MutQuery2<'a, C0, C1, F> {
    pub fn into_iter(self) -> MutIter<'a, Self> {
        MutIter::new(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// A layout error occured
    ///
   LayoutError(LayoutError),

    ///
    /// Could not allocate memory
    ///
    MemoryAllocationFailed,

    ///
    /// There were no registered components
    ///
    NoComponents,

    ///
    /// No entity for this ID
    ///
    NoEntity,

    ///
    /// The number of supported components has been exceeded
    ///
    TooManyComponents,

    ///
    /// Tried to insert an unregistered component
    ///
    UnregisteredComponent,
}

impl From<LayoutError> for Error {
    fn from(e: LayoutError) -> Self {
        Error::LayoutError(e)
    }
}

///
/// A builder for the world
///
pub struct WorldBuilder {
    ///
    /// The columns
    ///
    columns: Vec<Column>,

    ///
    /// The capacity
    ///
    capacity: usize,
}

impl WorldBuilder {

    const MAX_COMPONENTS: usize = 32;

    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            capacity: 0,
        }
    }

    ///
    /// Sets the capacity
    ///
    pub fn with_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    ///
    /// Registers a component
    ///
    pub fn register_component<T: Component>(&mut self) -> Result<(), Error>{
        if self.columns.len() == Self::MAX_COMPONENTS {
            Err(Error::TooManyComponents)
        } else {
            self.columns.push(Column {
                name: T::get_component_name(),
                mask: 0,
                offset: 0,
                layout: Layout::new::<T>(),
                drop_handle: Box::new(|ptr| unsafe {
                    std::ptr::drop_in_place(ptr as * mut T);
                }),
            });
            Ok(())
        }
    }

    ///
    /// Builds an empty world
    ///
    pub fn build(mut self) -> Result<World, Error> {
        if self.columns.len() == 0 {
            Err(Error::NoComponents)
        } else {
            let mut mask = 0x01;
            let mut header_layout = Layout::new::<u32>().pad_to_align();
            let mut layout = header_layout.clone();
            for i in 0..self.columns.len() {
                let (column_layout, offset) = layout.extend(self.columns[i].layout)?;
                self.columns[i].mask = mask;
                self.columns[i].offset = offset;
                mask <<= 1;
                layout = column_layout;
            }
            let mut world = World {
                columns: self.columns,
                data: NonNull::dangling(),
                amount: 0,
                capacity: 0,
                layout: layout.pad_to_align(),
                header_layout,
                free_list: BinaryHeap::new(),
            };
            if self.capacity > 0 {
                world.set_capacity(self.capacity)?;
            }
            Ok(world)
        }
    }
}

///
/// A descriptor of a component
///
struct Column {
    ///
    /// The name of the component
    ///
    name: &'static str,

    ///
    /// The bit field mask
    ///
    mask: u32,

    ///
    /// The offset where the component is stored
    ///
    offset: usize,

    ///
    /// The layout
    ///
    layout: Layout,

    ///
    /// The drop handle
    ///
    drop_handle: Box<dyn Fn(* mut u8)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Component1 {
        a: i16,
        b: u32,
    }

    impl Component for Component1 {
        fn get_component_name() -> &'static str {
            "component1"
        }
    }

    #[derive(Debug, PartialEq)]
    struct Component2 {
        c: i16
    }

    impl Component for Component2 {
        fn get_component_name() -> &'static str {
            "component2"
        }
    }

    #[test]
    fn test_build() {
        let mut builder = WorldBuilder::new();
        builder.register_component::<Component1>().unwrap();
        builder.register_component::<Component2>().unwrap();
        let world = builder.build().unwrap();
        assert_eq!(world.columns.len(), 2);
        assert_eq!(world.columns[0].name, "component1");
        assert_eq!(world.columns[1].name, "component2");
        assert_eq!(world.amount, 0);
        assert_eq!(world.capacity, 0);
    }

    #[test]
    fn test_build_and_alloc() {
        let mut builder = WorldBuilder::new();
        builder.with_capacity(10);
        builder.register_component::<Component1>().unwrap();
        builder.register_component::<Component2>().unwrap();
        let world = builder.build().expect("Expected world");
        assert_eq!(world.columns.len(), 2);
        assert_eq!(world.columns[0].name, "component1");
        assert_eq!(world.columns[1].name, "component2");
        assert_eq!(world.amount, 0);
        assert_eq!(world.capacity, 10);
    }

    #[test]
    fn test_realloc() {
        let mut builder = WorldBuilder::new();
        builder.with_capacity(10);
        builder.register_component::<Component1>().unwrap();
        builder.register_component::<Component2>().unwrap();
        let mut world = builder.build().expect("Expected world");
        assert_eq!(world.columns.len(), 2);
        assert_eq!(world.columns[0].name, "component1");
        assert_eq!(world.columns[1].name, "component2");
        assert_eq!(world.amount, 0);
        assert_eq!(world.capacity, 10);
        world.set_capacity(5).expect("Expected set_capacity to 5 success");
        assert_eq!(world.capacity, 5);
        world.set_capacity(15).expect("Expected set_capacity to 15 success");
        assert_eq!(world.capacity, 15);
    }

    #[test]
    fn test_query1() {
        let mut builder = WorldBuilder::new();
        builder.with_capacity(10);
        builder.register_component::<Component1>().unwrap();
        builder.register_component::<Component2>().unwrap();
        let mut world = builder.build().expect("Expected world");
        let entity1 = world.insert1(Component1{
            a: 1,
            b: 2,
        }).unwrap();
        let entity2 = world.insert1(Component1{
            a: 2,
            b: 4,
        }).unwrap();
        let mut iter = world.query1(|_: &Component1| true).unwrap().into_iter();
        assert_eq!(Some(&Component1 {
            a: 1,
            b: 2
        }), iter.next());
        assert_eq!(Some(&Component1 {
            a: 2,
            b: 4
        }), iter.next());
        assert_eq!(None, iter.next());

        let mut iter = world.query1(|c: &Component1| c.a % 2 == 0).unwrap().into_iter();
        assert_eq!(Some(&Component1 {
            a: 2,
            b: 4
        }), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_mut_query1() {
        let mut builder = WorldBuilder::new();
        builder.with_capacity(10);
        builder.register_component::<Component1>().unwrap();
        builder.register_component::<Component2>().unwrap();
        let mut world = builder.build().expect("Expected world");
        let entity1 = world.insert1(Component1{
            a: 1,
            b: 2,
        }).unwrap();
        let entity2 = world.insert1(Component1{
            a: 2,
            b: 4,
        }).unwrap();
        let mut iter_mut = world.mut_query1(|c: &Component1| c.a % 2 == 0).unwrap().into_iter();
        let mutable_entity = iter_mut.next().unwrap();
        assert_eq!(2, mutable_entity.a);
        assert_eq!(4, mutable_entity.b);
        mutable_entity.a *= 2;
        mutable_entity.b *= 2;
        assert_eq!(None, iter_mut.next());

        let mut iter = world.query1(|_: &Component1| true).unwrap().into_iter();
        assert_eq!(Some(&Component1 {
            a: 1,
            b: 2
        }), iter.next());
        assert_eq!(Some(&Component1 {
            a: 4,
            b: 8
        }), iter.next());
        assert_eq!(None, iter.next());

        let mut iter = world.query1(|c: &Component1| c.a % 2 == 0).unwrap().into_iter();
        assert_eq!(Some(&Component1 {
            a: 4,
            b: 8
        }), iter.next());
        assert_eq!(None, iter.next());
    }
}