## Prototype Pattern

The prototype pattern is a creational design pattern that creates new objects by cloning an existing object, known as the prototype. This
pattern is useful when the cost of creating an object from scratch is more expensive than copying an existing instance. By using prototypes,
the system can create new objects efficiently and dynamically, promoting flexibility and reducing the need for subclassing.

### Go Example

```go
package main

import "fmt"

type Clonable interface {
	Clone() Clonable
}

type Person struct {
	Name string
	Age  int
}

func (p *Person) Clone() Clonable {
	clone := *p
	return &clone
}

func main() {
	original := &Person{"John", 25}
	clone := original.Clone().(*Person)

	fmt.Println(original) // Output: &{John 25}
	fmt.Println(clone)    // Output: &{John 25}
}
```

### Perl Example

```perl
package Person;

use strict;
use warnings;

sub new {
    my ($class, $name, $age) = @_;
    return bless { name => $name, age => $age }, $class;
}

sub clone {
    my $self = shift;
    return bless { %$self }, ref($self);
}

package main;

my $original = Person->new("John", 25);
my $clone = $original->clone();

use Data::Dumper;
print Dumper($original); # Output: $VAR1 = { 'name' => 'John', 'age' => 25 };
print Dumper($clone);    # Output: $VAR1 = { 'name' => 'John', 'age' => 25 };
```

### Python Example

```python
import copy

class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def clone(self):
        return copy.deepcopy(self)

original = Person("John", 25)
clone = original.clone()

print(original.__dict__) # Output: {'name': 'John', 'age': 25}
print(clone.__dict__)    # Output: {'name': 'John', 'age': 25}
```

### Ruby Example

```ruby
class Person
  attr_accessor :name, :age

  def initialize(name, age)
    @name = name
    @age = age
  end

  def clone
    Marshal.load(Marshal.dump(self))
  end
end

original = Person.new("John", 25)
clone = original.clone

puts original.inspect # Output: #<Person:0x0000560ebf10e6c0 @name="John", @age=25>
puts clone.inspect    # Output: #<Person:0x0000560ebf10e518 @name="John", @age=25>
```

### Rust Example

```rust
#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn clone_box(&self) -> Box<Person> {
        Box::new(self.clone())
    }
}

fn main() {
    let original = Person::new("John", 25);
    let clone = original.clone_box();

    println!("{:?}", original); // Output: Person { name: "John", age: 25 }
    println!("{:?}", clone);    // Output: Person { name: "John", age: 25 }
}
```
