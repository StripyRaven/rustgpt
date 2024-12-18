`NaiveDateTime` is a type provided by the `chrono` crate for representing a date and time without any reference to a specific timezone. It contains information about the year, month, day, hour, minute, second, and microsecond.

Here's how you can use it:

```rust
extern crate chrono;

use chrono::NaiveDateTime;

fn main() {
    let dt = NaiveDateTime::parse_from_str("2016-05-18T10:49:37.123456", "%Y-%m-%dT%H:%M:%S%.f")
        .unwrap();

    println!("Date and Time is: {:?}", dt);
}
```

This will output the following:

```
Date and Time is: 2016-05-18T10:49:37.123456
```

Best practices in using `NaiveDateTime`:

1. Parse date/time strings: The `parse_from_str` function helps parse strings into a `NaiveDateTime`. However, it can fail if the string does not match the provided format, so you should use `unwrap()` or `match` to handle this.

2. Use correct format string: Always specify a precise format string when parsing date/time strings. This ensures that your code is robust and can correctly interpret different kinds of date/time representations.

3. Do not include timezone information: As mentioned earlier, `NaiveDateTime` does not contain any timezone information. If you need to include timezone information, use the `DateTime` type provided by the same crate instead.

4. Use methods for manipulation: The `NaiveDateTime` type provides many useful methods for manipulating date and time, such as adding or subtracting time units, formatting dates in different ways, and comparing dates and times. These methods can help you write cleaner and more efficient code.

Remember, when working with dates and times in Rust, always choose the right tool for the job. If `NaiveDateTime` is not sufficient for your needs, consider using `DateTime` instead.

For your `chrono` example:

```rust
use chrono::{DateTime, Utc, FixedOffset, Local};
use std::time::SystemTime;

// Convert from FixedOffset to DateTime<Utc>
let fixed_dt: DateTime<FixedOffset> = ...;
let utc_dt: DateTime<Utc> = DateTime::<Utc>::from(fixed_dt);

// Convert from Local to DateTime<Utc>
let local_dt: DateTime<Local> = ...;
let utc_dt: DateTime<Utc> = DateTime::<Utc>::from(local_dt);

// Convert from SystemTime to DateTime<Utc>
let system_time: SystemTime = ...;
let utc_dt: DateTime<Utc> = DateTime::<Utc>::from(system_time);
```
