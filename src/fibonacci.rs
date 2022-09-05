pub fn nth_fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 2;
    }

    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}
