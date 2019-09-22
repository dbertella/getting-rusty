struct NumSeq<T>
    where T: Fn(&mut Vec<usize>)
{
    stack: Vec<usize>,
    calculation: T,
}

impl<T> NumSeq<T>
    where T: Fn(&mut Vec<usize>)
{
    fn new(calculation: T) -> NumSeq<T>
    {
        Self {
            stack: vec!(),
            calculation,
        }
    }

    fn initial(mut self, num: usize) -> Self
    {
        self.stack.push(num);
        self
    }
}

impl<T> Iterator for NumSeq<T>
    where T: Fn(&mut Vec<usize>)
{
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        (self.calculation)(&mut self.stack);
        Some(self.stack)
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

    dbg!(fibonacci.stack);

    // for (i, n) in fibonacci.iter().take(10).enumerate()
    // {
    //     println!("[{}] {}", i, n);
    // }
    
    // println!("- - -");
    
    // for (i, n) in fibonacci.iter().take(10).enumerate()
    // {
    //     println!("[{}] {}", i, n);
    // }
    
    // println!("- - -");
    
    // let tribonacci = NumSeq::new(
    //     |stack|
    //     {
    //         let third = stack.pop().unwrap();
    //         let second = stack.pop().unwrap();
    //         let first = stack.pop().unwrap();
    //         stack.push(second);
    //         stack.push(third);
    //         stack.push(first + second + third);
    //     })
    //     .initial(0)
    //     .initial(1)
    //     .initial(1);
    
    // for (i, n) in tribonacci.iter().take(10).enumerate()
    // {
    //     println!("[{}] {}", i, n);
    // }
}