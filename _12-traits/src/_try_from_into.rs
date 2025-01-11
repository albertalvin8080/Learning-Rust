struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(format!("{} is not even.", value))
        }
    }
}

pub fn run() {
    match EvenNum::try_from(8) {
        Ok(even) => println!("{}", even.0),
        Err(msg) => println!("{}", msg), 
    }

    match EvenNum::try_from(5) {
        Ok(even) => println!("{}", even.0),
        Err(msg) => println!("{}", msg), 
    }

    let r: Result<EvenNum, String> = 67_i32.try_into();
    match r {
        Ok(even) => println!("{}", even.0),
        Err(msg) => println!("{}", msg), 
    }

    let r: Result<EvenNum, String> = 68_i32.try_into();
    match r {
        Ok(even) => println!("{}", even.0),
        Err(msg) => println!("{}", msg), 
    }
}
