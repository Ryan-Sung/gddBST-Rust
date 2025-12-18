use crate::node::Node;
use core::panic;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Handle {
    // keep primitive or ?
    index: usize,
    generation: u32,
}

impl Handle {
    pub(crate) fn new(index: usize, generation: u32) -> Self {
        return Self { index, generation };
    }
    pub(crate) fn index(&self) -> usize {
        return self.index;
    }
    pub(crate) fn generation(&self) -> u32 {
        return self.generation;
    }
}

struct Slot<T> {
    value: Option<T>,
    generation: u32,
}
pub(crate) struct Arena<T> {
    slots: Vec<Slot<T>>,
    free_list: Vec<usize>,
}

impl<T> Arena<T> {
    const INITIAL_GENERATION: u32 = 0;

    pub(crate) fn new() -> Self {
        Arena {
            slots: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub(crate) fn get(&self, handle: Handle) -> Option<&T> {
        let slot = self.slots.get(handle.index)?;
        if slot.generation != handle.generation {
            return None;
        }

        slot.value.as_ref()
    }

    pub(crate) fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.index)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_mut()
    }

    pub(crate) fn alloc(&mut self, value: T) -> Handle {
        if let Some(index) = self.free_list.pop() {
            // non-empty
            let generation = self.slots[index].generation;
            self.slots[index] = Slot {
                value: Some(value),
                generation,
            };
            Handle { index, generation }
        } else {
            // empty
            self.slots.push(Slot {
                value: Some(value),
                generation: Self::INITIAL_GENERATION,
            });
            Handle {
                index: self.slots.len() - 1,
                generation: Self::INITIAL_GENERATION,
            }
        }
    }

    pub(crate) fn free(&mut self, handle: Handle) -> Option<T> {
        let slot = self.slots.get_mut(handle.index)?;

        if slot.generation != handle.generation {
            return None;
        }

        let value = slot.value.take()?;
        slot.generation += 1;
        self.free_list.push(handle.index);
        return Some(value);
    }

    pub(crate) fn is_alive(&self, handle: Handle) -> bool {
        self.slots
            .get(handle.index)
            .is_some_and(|slot| slot.generation == handle.generation && slot.value.is_some())
    }

    pub(crate) fn contains(&self, handle: Handle) -> bool {
        self.slots.get(handle.index).is_some()
    }
}

/*
修
bst.rs
arena.rs
*/
