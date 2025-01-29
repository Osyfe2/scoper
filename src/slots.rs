use std::mem::MaybeUninit;


/// Models a static filo stack
/// You can get the content of certain slots
/// Push might overflow
/// 
/// `Slots<10>`
/// |x|x|x|?|?|?|?|_|_|_|
/// 
/// x: filled -> from `0` till `first_unknown`
/// ?: unknown -> from `first_unknown` till `first_empty`
/// _: empty -> from `first_empty` till `SIZE`
/// 
#[derive(Debug)]
pub struct Slots<T> {
    //data: [MaybeUninit<T>; SIZE],
    //first_empty: usize, //len
    first_unknown: usize,
    unclaimed_empty_count: usize,
    data: Vec<MaybeUninit<T>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotIndex
{
    index: usize
}

impl SlotIndex
{
    pub(super) unsafe fn duplicate(&self) -> Self
    {
        Self { index: self.index }
    }
}

impl <T: Copy> Slots<T>
{
    pub fn push(&mut self, item: T) -> SlotIndex
    {
        /*
        if self.full()
        {
            None
        } 
        else
        {
            self.data[self.first_empty].write(item);
            let slot_index = SlotIndex { index: self.first_empty };
            self.first_empty += 1;
            Some(slot_index)
        }
        */
        let slot_index = SlotIndex { index: self.data.len() };
        self.data.push(MaybeUninit::new(item));
        slot_index
    }

    pub fn take(&mut self, SlotIndex{index}: SlotIndex) -> T
    {
        let datum = 
        unsafe { core::hint::assert_unchecked(index < self.data.len());
            self.data[index].assume_init()
        };

        self.unclaimed_empty_count += 1;
        self.first_unknown = self.first_unknown.min(index);
        if self.first_unknown + self.unclaimed_empty_count == self.data.len()
        {
            //self.first_empty = self.first_unknown;
            self.data.truncate(self.first_unknown);
            self.unclaimed_empty_count = 0;
        }

        datum
    }

    pub fn get(&self, &SlotIndex{index}: &SlotIndex) -> &T
    {
        unsafe { core::hint::assert_unchecked(index < self.data.len());
            self.data[index].assume_init_ref()
        }
    }

    pub fn get_mut(&mut self, &SlotIndex{index}: &SlotIndex) -> &mut T
    {
        unsafe { core::hint::assert_unchecked(index < self.data.len());
            self.data[index].assume_init_mut()
        }
    }

    pub fn filled_slots(&self) -> usize
    {
        self.data.len() - self.unclaimed_empty_count
    }

    pub fn available_slots(&self) -> usize
    {
        self.data.capacity() - self.data.len()
    }

    pub fn is_empty(&self) -> bool
    {
        self.data.is_empty()
    }

    pub fn full(&self) -> bool
    {
        self.available_slots() == 0
    }
}


impl <T> Default for Slots<T>
{
    fn default() -> Self {
        Self { data: Vec::new(), first_unknown: Default::default(), unclaimed_empty_count: Default::default() }
    }
}