extern crate load_input;

fn main() {
    let mul_a = 16807;
    let mul_b = 48271;
    let div = 2147483647;

    let input = load_input::load_input();
    let (a, b) = {
        let mut nums = input.split_whitespace().flat_map(str::parse::<u64>);
        (nums.next().expect("start for a"), nums.next().expect("start for b"))
    };

    println!("Starting with a={}, b={}", a, b);
    
    let iter_a = unfold::Unfold::new(a, |st|{ *st = *st * mul_a % div; Some(*st) });
    let iter_b = unfold::Unfold::new(b, |st|{ *st = *st * mul_b % div; Some(*st) });
    let matches = iter_a.zip(iter_b).take(40_000_000).filter(|&(a, b)| (a ^ b) & 0xffff == 0).count();
    println!("Got {} matches", matches);

    let iter_a = unfold::Unfold::new(a, |st|{ *st = *st * mul_a % div; Some(*st) }).filter(|&n|n % 4 == 0);
    let iter_b = unfold::Unfold::new(b, |st|{ *st = *st * mul_b % div; Some(*st) }).filter(|&n|n % 8 == 0);
    let matches = iter_a.zip(iter_b).take(5_000_000).filter(|&(a, b)| (a ^ b) & 0xffff == 0).count();
    println!("Got {} matches", matches);
}

// This unfold implementation is borrowed from the Rust standard library. It was deprecated around version 1.2.
// 
mod unfold {
    #[derive(Clone)]
    pub struct Unfold<St, F> {
        f: F,
        /// Internal state that will be passed to the closure on the next iteration
        pub state: St,
    }

    impl<A, St, F> Unfold<St, F> where F: FnMut(&mut St) -> Option<A> {
        /// Creates a new iterator with the specified closure as the "iterator
        /// function" and an initial state to eventually pass to the closure
        #[inline]
        pub fn new(initial_state: St, f: F) -> Unfold<St, F> {
            Unfold {
                f: f,
                state: initial_state
            }
        }
    }

    impl<A, St, F> Iterator for Unfold<St, F> where F: FnMut(&mut St) -> Option<A> {
        type Item = A;

        #[inline]
        fn next(&mut self) -> Option<A> {
            (self.f)(&mut self.state)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            // no possible known bounds at this point
            (0, None)
        }
    }

}