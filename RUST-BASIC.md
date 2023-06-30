
# SCALER TYPES

### signed & unsigned type
    u8          i8
    u16         i16
    u32         i32
    u64         i64
    u128        i128
    usize       isize

### Rust variables can be declared as:
Decimal         100000
Hex             0xdeadbeef
Octal           0o77543211
Binary          0b11110011
Byte (u8 only)  b'A'


### floating point types
      f32
      f64 //default
### example declaration
```
let x: u16 = 5;
let y: f32 = 3.14;
```
same can be declare as suffix
```
let x = 5u16;
let y = 3.14f32;
```
or using more readable use "_"
```
let x = 5_u16;
let y = 3.14_f32;
```

### boolean types
```
//boolean is not integer unless it declared as bool as u8
let x: bool = true; 
```

### character type
```
//char size is 4 bytes (32 bits)
//makes array of charecters USC-4 / UTF-32
let my_char = 'abcd';
```

# COMPOUND TYPES


### Tuple - stores multiple values any type
```
//maximum can be use 12 elements - above that functionality will be problem
//declaring
let info = (1, 3.3, 999);
let info: (u8, f64, i32) = (1, 3.3, 999);

//accesing tuples - 1
let info = (1, 3.3, 999);
let jets = info.0;
let fuel = info.1;
let ammo = info.2;

//accesing tuples - 2
let (jets,fuel, ammo) = info;

```
### Array
```
// arrays limits is 32 - store in stacks by default
let buf = [1, 2, 3];

let buf[0; 3];

// means: number of '3' elements are 'u8' type 
let buf: [u8; 3] = [1, 2, 3];
```

# CONTROL FLOWS

### if contitions 
```
//traditional if statements
  if num == 5 {
    msg = "five";
  } else if num == 4 {
    msg = "four";  
  } else {
    msg = "other";
  }

//new way in rust
  num = if num == 5 {
    "five"
  } else if num == 4 {
    "four"
  } else {
    "other"
  };
 
NOTE:
0. there is no semicolon ";"
1. ";" only at the end must 
2. return can not be called in between
3. all the blocks return same type

// instead in (a) ? b : c; can be use as:
num = if a { b } else { c };
```

### loop { } : unconditional loop
```
//ex1
loop {
    do_stuff
    break;
  }
//ex2
loop {
    loop {
        loop {
            break;
          }
      }
  }
//if you want to break the first loop then add a TICK "'" and assign a name to that loop

'bob: loop {
    loop {
        loop {
            break 'bob; //call as: tick bob or 'bob
          }
      }
  }

```
### for loop
```
for num in [7, 8, 9].iter() {
    // do stuff with num
  }

  let arrau = [(1,2), (3,4)]
  for (x,y) in array.iter() {
    // do stuff with x & y
    }

  for num in 0..50 {   //counts 0 to 49
      //do stuff with num
    }

  for num in 0..=50 {   //counts 0 to 50
      //do stuff with num
    }
```

# Strings - 6types string but 2 is imp

# Ownership
















