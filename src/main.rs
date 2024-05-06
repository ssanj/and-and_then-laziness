use std::{thread, time::Duration};

type R = Result<(), String>;


fn main() {
    println!("------and-------");
    first_and_second().unwrap();
    println!("----------------");
    define_then_call_with_and().unwrap();
    println!();
    println!();
    println!("----and_then----");
    first_and_then_second().unwrap();
    println!("----------------");
    define_then_call_with_and_then().unwrap();
    println!();
    println!();
    println!("----- failure and ------");
    let result1 = first_failure_and_second();
    println!("{:?}", result1);
    println!("----- failure and_then ------");
    let result2 = first_failure_and_then_second();
    println!("{:?}", result2)
}


fn first_and_second() -> R {
  println!("first_and_second");
  println!("calling first `and` second");
  first().and(second())
}

fn first_failure_and_second() -> R {
  println!("first_fail_and_second");
  first_fail().and(second())
}

fn first_failure_and_then_second() -> R {
  println!("first_fail_and_then_second");
  first_fail().and_then(|_|second())
}

fn first_and_then_second() -> R {
  println!("first_and_then_second");
  println!("calling first `and_then` second");
  first().and_then(|_| second())
}


fn define_then_call_with_and() -> R {
  println!("define second and then first_and_second");
  println!("defining second");
  // calculate second before using `and`. Doing this will have side effects
  let second: Result<(), String> = second();

  println!("then calling `and` on first");
  first().and(second)
}

fn define_then_call_with_and_then() -> R {
  println!("define second and then first_and_then_second");
  println!("defining second");
  // calculate second before using `and`. Doing this will have side effects
  let second: Result<(), String> = second();

  println!("then calling `and_then` on first");
  first().and_then(|_| second)
}

fn first() -> R {
  println!("First: Sleeping for 2 seconds");
  thread::sleep(Duration::from_secs(2));
  println!("First complete");
  Ok(())
}


fn second() -> R {
  println!("Second complete");
  Ok(())
}

fn first_fail() -> R {
  Err("first failed".to_owned())
}
