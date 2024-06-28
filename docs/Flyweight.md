## Flyweight Pattern

The flyweight pattern is a structural design pattern that minimizes memory usage by sharing as much data as possible with similar objects.
It is particularly useful when dealing with a large number of objects that have many common attributes. By sharing common data between objects,
the flyweight pattern reduces redundancy and memory consumption, improving performance and scalability. This pattern involves creating a flyweight
object that contains shared data and a context object that contains unique data, enabling efficient management of a large number of objects.

### Go Example

```go
package main

import (
    "fmt"
)

type Flyweight struct {
    sharedState string
}

func (f *Flyweight) Operation(uniqueState string) {
    fmt.Printf("Flyweight: Displaying shared (%s) and unique (%s) state.\n", f.sharedState, uniqueState)
}

type FlyweightFactory struct {
    flyweights map[string]*Flyweight
}

func NewFlyweightFactory() *FlyweightFactory {
    return &FlyweightFactory{flyweights: make(map[string]*Flyweight)}
}

func (f *FlyweightFactory) GetFlyweight(sharedState string) *Flyweight {
    if flyweight, exists := f.flyweights[sharedState]; exists {
        return flyweight
    }
    flyweight := &Flyweight{sharedState: sharedState}
    f.flyweights[sharedState] = flyweight
    return flyweight
}

// Usage
func main() {
    factory := NewFlyweightFactory()
    flyweight1 := factory.GetFlyweight("Shared State 1")
    flyweight2 := factory.GetFlyweight("Shared State 1")
    flyweight3 := factory.GetFlyweight("Shared State 2")

    flyweight1.Operation("Unique State A")
    flyweight2.Operation("Unique State B")
    flyweight3.Operation("Unique State C")
}
```

### Perl Example

```perl
package Flyweight;
sub new {
    my ($class, $shared_state) = @_;
    return bless { shared_state => $shared_state }, $class;
}

sub operation {
    my ($self, $unique_state) = @_;
    print "Flyweight: Displaying shared ($self->{shared_state}) and unique ($unique_state) state.\n";
}

package FlyweightFactory;
sub new {
    my $class = shift;
    return bless { flyweights => {} }, $class;
}

sub get_flyweight {
    my ($self, $shared_state) = @_;
    $self->{flyweights}{$shared_state} ||= Flyweight->new($shared_state);
    return $self->{flyweights}{$shared_state};
}

# Usage
my $factory = FlyweightFactory->new;
my $flyweight1 = $factory->get_flyweight("Shared State 1");
my $flyweight2 = $factory->get_flyweight("Shared State 1");
my $flyweight3 = $factory->get_flyweight("Shared State 2");

$flyweight1->operation("Unique State A");
$flyweight2->operation("Unique State B");
$flyweight3->operation("Unique State C");
```

### Python Example

```python
class Flyweight:
    def __init__(self, shared_state):
        self.shared_state = shared_state

    def operation(self, unique_state):
        print(f"Flyweight: Displaying shared ({self.shared_state}) and unique ({unique_state}) state.")

class FlyweightFactory:
    _flyweights = {}

    def get_flyweight(self, shared_state):
        if shared_state not in self._flyweights:
            self._flyweights[shared_state] = Flyweight(shared_state)
        return self._flyweights[shared_state]

# Usage
factory = FlyweightFactory()
flyweight1 = factory.get_flyweight("Shared State 1")
flyweight2 = factory.get_flyweight("Shared State 1")
flyweight3 = factory.get_flyweight("Shared State 2")

flyweight1.operation("Unique State A")
flyweight2.operation("Unique State B")
flyweight3.operation("Unique State C")
```

### Ruby Example

```ruby
class Flyweight
  def initialize(shared_state)
    @shared_state = shared_state
  end

  def operation(unique_state)
    puts "Flyweight: Displaying shared (#{@shared_state}) and unique (#{unique_state}) state."
  end
end

class FlyweightFactory
  def initialize
    @flyweights = {}
  end

  def get_flyweight(shared_state)
    @flyweights[shared_state] ||= Flyweight.new(shared_state)
  end
end

# Usage
factory = FlyweightFactory.new
flyweight1 = factory.get_flyweight("Shared State 1")
flyweight2 = factory.get_flyweight("Shared State 1")
flyweight3 = factory.get_flyweight("Shared State 2")

flyweight1.operation("Unique State A")
flyweight2.operation("Unique State B")
flyweight3.operation("Unique State C")
```

### Rust Example

```rust
use std::collections::HashMap;
use std::rc::Rc;

struct Flyweight {
    shared_state: String,
}

impl Flyweight {
    fn new(shared_state: &str) -> Flyweight {
        Flyweight { shared_state: shared_state.to_string() }
    }

    fn operation(&self, unique_state: &str) {
        println!("Flyweight: Displaying shared ({}) and unique ({}) state.", self.shared_state, unique_state);
    }
}

struct FlyweightFactory {
    flyweights: HashMap<String, Rc<Flyweight>>,
}

impl FlyweightFactory {
    fn new() -> FlyweightFactory {
        FlyweightFactory { flyweights: HashMap::new() }
    }

    fn get_flyweight(&mut self, shared_state: &str) -> Rc<Flyweight> {
        self.flyweights.entry(shared_state.to_string())
            .or_insert_with(|| Rc::new(Flyweight::new(shared_state)))
            .clone()
    }
}

// Usage
fn main() {
    let mut factory = FlyweightFactory::new();
    let flyweight1 = factory.get_flyweight("Shared State 1");
    let flyweight2 = factory.get_flyweight("Shared State 1");
    let flyweight3 = factory.get_flyweight("Shared State 2");

    flyweight1.operation("Unique State A");
    flyweight2.operation("Unique State B");
    flyweight3.operation("Unique State C");
}
```
