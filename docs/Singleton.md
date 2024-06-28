## Singleton Pattern

The singleton pattern is a creational design pattern that ensures a class has only one instance and provides a global point of access to it.
This pattern is useful for managing shared resources or coordinating actions across an application. By controlling the instantiation process,
the singleton pattern prevents the creation of multiple instances, ensuring that all requests for the instance are directed to the same object.
This approach promotes resource management and consistency, making it easier to manage and coordinate shared resources effectively.

### Go Example

```go
package main

import (
	"fmt"
	"sync"
)

type Singleton struct{}

var instance *Singleton
var once sync.Once

func GetInstance() *Singleton {
	once.Do(func() {
		instance = &Singleton{}
	})
	return instance
}

func main() {
	singleton1 := GetInstance()
	singleton2 := GetInstance()

	fmt.Println(singleton1 == singleton2)  // Output: true
}
```

### Perl Example

```perl
package Singleton;

my $instance;

sub new {
    my $class = shift;
    unless (defined $instance) {
        $instance = bless {}, $class;
    }
    return $instance;
}

# Usage
my $singleton1 = Singleton->new();
my $singleton2 = Singleton->new();

print $singleton1 == $singleton2 ? "true\n" : "false\n";  # Output: true
```

### Python Example

```python
class Singleton:
    _instance = None

    def __new__(cls, *args, **kwargs):
        if not cls._instance:
            cls._instance = super(Singleton, cls).__new__(cls, *args, **kwargs)
        return cls._instance

# Usage
singleton1 = Singleton()
singleton2 = Singleton()

print(singleton1 is singleton2)  # Output: True
```

### Ruby Example

```ruby
class Singleton
  @instance = nil

  private_class_method :new

  def self.instance
    @instance ||= new
  end
end

# Usage
singleton1 = Singleton.instance
singleton2 = Singleton.instance

puts singleton1.equal?(singleton2)  # Output: true
```

### Rust Example

```rust
use std::sync::{Arc, Mutex};
use std::sync::Once;

struct Singleton {
    // Add fields here
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            // Initialize fields here
        }
    }
}

static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
static ONCE: Once = Once::new();

fn singleton_instance() -> Arc<Mutex<Singleton>> {
    unsafe {
        ONCE.call_once(|| {
            let singleton = Singleton::new();
            SINGLETON = Some(Arc::new(Mutex::new(singleton)));
        });
        SINGLETON.clone().unwrap()
    }
}

fn main() {
    let singleton1 = singleton_instance();
    let singleton2 = singleton_instance();

    println!("{}", Arc::ptr_eq(&singleton1, &singleton2));  // Output: true
}
```
