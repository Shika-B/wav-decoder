// Takes a slice of bytes and returns an array composed
// of the element at the specified index and the element following it
pub fn slice_to_array2(index: usize, slice: &[u8]) -> [u8; 2] {
    if slice.len() < index + 1 {
        panic!("Invalid header")
    }
    [slice[index], slice[index + 1]]
}

// Same thing but with four elements
pub fn slice_to_array4(index: usize, slice: &[u8]) -> [u8; 4] {
    if slice.len() < index + 3 {
        panic!("Invalid header")
    }
    [
        slice[index],
        slice[index + 1],
        slice[index + 2],
        slice[index + 3],
    ]
}

