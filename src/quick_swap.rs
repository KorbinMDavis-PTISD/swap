/// Quick Swap Function that swaps two unsigned 32-bit integers using XOR
pub fn quick_swap(mut a:[u32; 2] ) -> [u32;2] {
    a[0] = a[0] ^ a[1];
    a[1] = a[0] ^ a[1];
    a[0] = a[0] ^ a[1];

    return a;
}
