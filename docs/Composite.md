## Composite Pattern

The composite pattern is a structural design pattern that enables you to compose objects into tree structures to represent part-whole hierarchies.
This pattern allows clients to treat individual objects and compositions of objects uniformly, simplifying the client code. By implementing a common
interface for both simple and complex objects, the composite pattern makes it easier to work with recursive structures and complex hierarchies.
This approach promotes flexibility and reusability, enabling the creation of complex structures with ease and facilitating operations on these structures.


### Go Example

```go
package main

import "fmt"

type Component interface {
    Operation()
}

type Leaf struct{}

func (l *Leaf) Operation() {
    fmt.Println("Leaf operation")
}

type Composite struct {
    children []Component
}

func (c *Composite) Add(component Component) {
    c.children = append(c.children, component)
}

func (c *Composite) Remove(component Component) {
    for i, child := range c.children {
        if child == component {
            c.children = append(c.children[:i], c.children[i+1:]...)
            break
        }
    }
}

func (c *Composite) Operation() {
    fmt.Println("Composite operation")
    for _, child := range c.children {
        child.Operation()
    }
}

func main() {
    leaf1 := &Leaf{}
    leaf2 := &Leaf{}

    composite := &Composite{}
    composite.Add(leaf1)
    composite.Add(leaf2)
    composite.Operation()
}
```

### Perl Example

```perl
package Component;
sub new { bless {}, shift }
sub operation { die "Abstract method\n" }

package Leaf;
our @ISA = qw(Component);
sub operation { print "Leaf operation\n" }

package Composite;
our @ISA = qw(Component);
sub new {
    my $class = shift;
    my $self = { _children => [] };
    bless $self, $class;
    return $self;
}

sub add {
    my ($self, $component) = @_;
    push @{$self->{_children}}, $component;
}

sub remove {
    my ($self, $component) = @_;
    @{$self->{_children}} = grep { $_ != $component } @{$self->{_children}};
}

sub operation {
    my $self = shift;
    print "Composite operation\n";
    $_->operation for @{$self->{_children}};
}

# Usage
my $leaf1 = Leaf->new;
my $leaf2 = Leaf->new;
my $composite = Composite->new;
$composite->add($leaf1);
$composite->add($leaf2);
$composite->operation;
```

### Python Example

```python
class Component:
    def operation(self):
        pass

class Leaf(Component):
    def operation(self):
        print("Leaf operation")

class Composite(Component):
    def __init__(self):
        self._children = []

    def add(self, component):
        self._children.append(component)

    def remove(self, component):
        self._children.remove(component)

    def operation(self):
        print("Composite operation")
        for child in self._children:
            child.operation()

# Usage
leaf1 = Leaf()
leaf2 = Leaf()
composite = Composite()
composite.add(leaf1)
composite.add(leaf2)
composite.operation()
```

### Ruby Example

```ruby
class Component
  def operation
    raise NotImplementedError, 'Subclasses must override this method'
  end
end

class Leaf < Component
  def operation
    puts 'Leaf operation'
  end
end

class Composite < Component
  def initialize
    @children = []
  end

  def add(component)
    @children << component
  end

  def remove(component)
    @children.delete(component)
  end

  def operation
    puts 'Composite operation'
    @children.each(&:operation)
  end
end

# Usage
leaf1 = Leaf.new
leaf2 = Leaf.new
composite = Composite.new
composite.add(leaf1)
composite.add(leaf2)
composite.operation
```

### Rust Example

```rust
trait Component {
    fn operation(&self);
}

struct Leaf;

impl Component for Leaf {
    fn operation(&self) {
        println!("Leaf operation");
    }
}

struct Composite {
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn new() -> Composite {
        Composite { children: Vec::new() }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }

    fn remove(&mut self, component: *const dyn Component) {
        self.children.retain(|&ref c| &**c as *const _ != component);
    }
}

impl Component for Composite {
    fn operation(&self) {
        println!("Composite operation");
        for child in &self.children {
            child.operation();
        }
    }
}

fn main() {
    let leaf1 = Box::new(Leaf);
    let leaf2 = Box::new(Leaf);

    let mut composite = Composite::new();
    composite.add(leaf1);
    composite.add(leaf2);
    composite.operation();
}
```
