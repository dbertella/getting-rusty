struct NumSeq<T>
    where T: Fn(&mut Vec<usize>)
{
    calculation: T,
    initial_values: Vec<usize>,
}

impl<T> NumSeq<T>
    where T: Fn(&mut Vec<usize>)
{
    fn new(calculation: T) -> Self
    {
        Self {
            calculation,
            initial_values: vec!(),
        }
    }

    fn iter(&self) -> NumSeqIter<'_, T>
    {
        NumSeqIter::new(&self)
    }

    fn initial(mut self, num: usize) -> Self
    {
        self.initial_values.push(num);
        self
    }
    
}

struct NumSeqIter<'a, T>
    where T: Fn(&mut Vec<usize>)
{
    stack: Vec<usize>,
    calculation: &'a T,
}

impl<'a, T> NumSeqIter<'a, T>
    where T: Fn(&mut Vec<usize>)
{
    fn new(initial: &'a NumSeq<T>) -> Self
    {
        Self {
            calculation: &initial.calculation,
            stack: initial.initial_values.to_vec(),
        }
    }
}

impl<T> Iterator for NumSeqIter<'_, T>
    where T: Fn(&mut Vec<usize>)
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item>
    {
        let value = *self.stack.get(0).unwrap();
        (self.calculation)(&mut self.stack);
        Some(value)
    }
}

fn main()
{
    let fibonacci = NumSeq::new(
        |stack|
        {
            let curr = stack.pop().unwrap();
            let prev = stack.pop().unwrap();
            print!("{} {}", curr, prev);
            stack.push(curr);
            stack.push(curr + prev);
        })
        .initial(0)
        .initial(1);

    for (i, n) in fibonacci.iter().take(10).enumerate()
    {
        println!("[{}] {}", i, n);
    }
    
    println!("- - -");
    
    for (i, n) in fibonacci.iter().take(10).enumerate()
    {
        println!("[{}] {}", i, n);
    }
    
    println!("- - -");
    
    let tribonacci = NumSeq::new(
        |stack|
        {
            let third = stack.pop().unwrap();
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            stack.push(second);
            stack.push(third);
            stack.push(first + second + third);
        })
        .initial(0)
        .initial(1)
        .initial(1);
    
    for (i, n) in tribonacci.iter().take(10).enumerate()
    {
        println!("[{}] {}", i, n);
    }
}