use rand::{distributions::range, Rng};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct User {
    name: String,
    id: i32,
}

#[derive(Debug)]
enum exam {
    pass(i32),
    fail(i32),
}

impl exam {
    fn validate_marks(&self, mark_ch: i32) {
        if let &exam::pass(value) = self {
            let x: bool = value > mark_ch;
            if x {
                println!("you are eligible for scholarship");
            } else {
                println!("you are not eligible for scholarship");
            }
        }
        if let &exam::fail(value) = self {
            let x: i32 = 40 - value;
            if x < 10 {
                println!("you are eligible to retake exam");
            } else {
                println!("you are not eligible for retake");
            }
        }
    }
}
fn main() {
    //using struct
    {
        let user_one = User {
            name: "huzail".to_string(),
            id: 10,
        };
        println!("{:?}", user_one);
    }

    //Exam

    {
        let mark = 80;
        println!(" please enter if you passed or failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        if input.trim() == "pass" {
            println!("enter your score please");
            let mut scorepass = String::new();
            io::stdin()
                .read_line(&mut scorepass)
                .expect("failed to read");
            let exam1 = exam::pass(scorepass.trim().parse().unwrap());
            println!("{:?}", exam1);
            exam1.validate_marks(mark)
        } else if input.trim() == "fail" {
            println!("enter your score please");
            let mut scorefail = String::new();
            io::stdin()
                .read_line(&mut scorefail)
                .expect("failed to read");
            let exam2 = exam::fail(scorefail.trim().parse().unwrap());
            println!("{:?}", exam2);
            exam2.validate_marks(mark)
        } else {
            println!("invalid input");
        }
    }

    //Two sum

    {
        println!("Two sum");
        let target = 9;
        let nums = vec![11, 2, 15, 7];
        for (i, num1) in nums.iter().enumerate() {
            for (j, num2) in nums.iter().skip(i + 1).enumerate() {
                if num1 + num2 == target {
                    println!("{:?} + {:?} = {:?} ", nums[i], nums[j + 1 + i], target);
                }
            }
        }
    }

    //Add two numbers

    {
        println!("Add two numbers 3 digit");
        let l1 = [2, 4, 3];
        let l2 = [5, 6, 4];

        let mut la = l1[0] + l2[0];
        let mut lb = l1[1] + l2[1];
        let mut lc = l1[2] + l2[2];
        if lc >= 10 {
            lb = lb + 1;
            lc = 0;
        }
        if lb >= 10 {
            la = la + 1;
            lb = 0;
        }
        println!("{}{}{}", la, lb, lc);
    }

    {
        //capitalize first letter
        println!("Capitalize first letter");

        let arr = ["hello", "huzail", "how", "are", "you"];
        for i in arr {
            let arr1 = &i[1..i.len()];
            let arr2 = &i[0..1].to_uppercase();
            println!("{}{}", arr2, arr1)
        }
    }

    {
        // reversing an array
        println!("reversing an array");
        let arr = vec![-3, -6, 1, 19, 2, 4, 7, 6, 5];
        let mut sortedarr = arr.to_vec();
        sortedarr.sort();
        sortedarr.reverse();
        println!("{:?}", sortedarr)
    }
    //random number generator
    let rng = rand::thread_rng().gen_range(5, 25);
    println!("The range is : {:?}", &rng);
    let mut numbers: Vec<usize> = Vec::new();
    for i in 0..rng {
        numbers.push(rand::thread_rng().gen_range(0, rng));
    }
    {
        //playing with fold and map
        let v = vec![1, 2, 3, 4, 5];
        let y: Vec<i32> = v.iter().map(|i| i * i).collect();
        println!("Squre of {:?}", y);

        let z = v.iter().fold(0, |sum, i| sum + i);
        println!("Sum of vector {}", z);

        let mean = v.iter().fold(0, |sum, i| sum + i) / v.len() as i32;
        println!("The mean is {}", mean);
    }
    {
        //std deviation
        let w = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = w.iter().fold(0.0, |sum, i| sum + i) / w.len() as f32;
        println!("The float mean is {}", mean);
        let std_dev = w
            .iter()
            .map(|i| (i - mean).powf(2.0))
            .fold(0.0, |sum, i| sum + i)
            / w.len() as f32;
        let std = std_dev.sqrt();
        println!("The std deviation is {:#?}", std)
    }
    {
        //using filter
        let records = vec![
            ("john", 52, 218),
            ("Doe", 29, 140),
            ("johnny", 34, 130),
            ("jay", 19, 150),
        ];
        // let old_enough: Vec<_> = records.iter().filter(|t| t.1 > age).collect();
        // let over_weightsnoldenough: Vec<_> = old_enough.iter().filter(|t| t.2 > wgt).collect();
        let over_weightsnoldenough: Vec<_> = records
            .iter()
            .filter(|t| t.1 > age)
            .filter(|t| t.2 > wgt)
            .collect();
        // println!("{:?}", old_enough);
        println!("{:?}", over_weightsnoldenough);
    }
    {
        //Duplicate keys
        let s = String::from("hello yr kese ho");
        let mut h: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            if h.contains_key(&c) == false {
                h.insert(c, 1);
            } else {
                println!("key {} is duplicate", c);
            }
        }
    }
    {
        //frequency of duplicate keys
        let s = String::from("hello yr kese ho");
        let mut h: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            println!("{}--", c);
            //  let count = h.entry(c).or_insert(0);
            // *count += 1;
            match h.entry(c) {
                Entry::Occupied(mut x) => {
                    *x.get_mut() += 1;
                }
                Entry::Vacant(_) => {
                    h.insert(c, 1);
                }
            }
        }
        println!("{:#?}", h);
    }
    {
        //fibonacci sequence
        let mut i = 0;
        let mut a = 0;
        let mut c = 0;
        let mut b = 1;

        while i < 10 {
            if i > 1 {
                c = b;
                b = b + a;
                a = c;
                println!("Fibonacci sequence {}", b);
                i = i + 1
            } else {
                println!("Fibonacci sequence {}", i);
                i = i + 1;
            }
        }
    }

    computemean(&numbers);
    computemedian(&mut numbers);
    computemode(&mut numbers);

    //fibonacci sequence
    let mut nths = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for nth in nths {
        println!("{} {}", nth, fib(nth))
    }
}
//Mean
fn computemean(n: &Vec<usize>) {
    let mut sum: usize = 0;
    for i in n {
        sum += i;
    }
    let mean = sum / n.len();
    println!(" mean {}", mean)
}
//Median
fn computemedian(n: &mut Vec<usize>) {
    let middle = n.len() / 2;
    n.sort();
    println!("{:?}", n);

    if n.len() % 2 == 0 {
        let mid = (middle + middle - 1) / 2;
        println!("Median {} ", mid);
    } else {
        println!(" Median {}", middle);
    }
}
//Mode
fn computemode(n: &mut Vec<usize>) {
    let mut map = HashMap::new();
    for num in n {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("map of occurence {:?}", map);
    let mut maxv = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > maxv {
            maxv = value;
            mode = *key;
        }
    }
    println!("Mode {}", mode);
    println!("value of mod {}", maxv);
}
//
static age: usize = 25;
static wgt: usize = 150;

//fibonacci sequence
fn fib(nths: i32) -> i32 {
    let mut prev = 0;
    let mut curr = 1;

    for i in 1..nths {
        let mut next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
