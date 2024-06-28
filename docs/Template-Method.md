## Template Method Pattern

The template method pattern is a behavioural design pattern that defines the skeleton of an algorithm in a base class, allowing subclasses to
redefine certain steps without changing the algorithm's structure. This pattern promotes code reuse by enabling common behaviour to be shared
across subclasses while allowing customization of specific steps. The template method pattern ensures that the overall algorithm remains
consistent, while subclasses provide the necessary variations, enhancing flexibility and maintainability.

### Go Example

```go
package main

import "fmt"

// AbstractClass
type AbstractClass interface {
	PrimitiveOperation1()
	PrimitiveOperation2()
	TemplateMethod()
}

// ConcreteClass
type ConcreteClass struct{}

func (c *ConcreteClass) PrimitiveOperation1() {
	fmt.Println("ConcreteClass: PrimitiveOperation1")
}

func (c *ConcreteClass) PrimitiveOperation2() {
	fmt.Println("ConcreteClass: PrimitiveOperation2")
}

func (c *ConcreteClass) TemplateMethod() {
	c.PrimitiveOperation1()
	c.PrimitiveOperation2()
}

func main() {
	concrete := &ConcreteClass{}
	concrete.TemplateMethod()
}
```

### Perl Example

```perl
package AbstractClass;

sub primitive_operation1 { die "Abstract method" }
sub primitive_operation2 { die "Abstract method" }

sub template_method {
    my $self = shift;
    $self->primitive_operation1();
    $self->primitive_operation2();
}

package ConcreteClass;
use parent 'AbstractClass';

sub primitive_operation1 {
    print "ConcreteClass: PrimitiveOperation1\n";
}

sub primitive_operation2 {
    print "ConcreteClass: PrimitiveOperation2\n";
}

package main;

my $concrete = ConcreteClass->new();
$concrete->template_method();
```

### Python Example

```python
from abc import ABC, abstractmethod

class AbstractClass(ABC):
    @abstractmethod
    def primitive_operation1(self):
        pass

    @abstractmethod
    def primitive_operation2(self):
        pass

    def template_method(self):
        self.primitive_operation1()
        self.primitive_operation2()

class ConcreteClass(AbstractClass):
    def primitive_operation1(self):
        print("ConcreteClass: PrimitiveOperation1")

    def primitive_operation2(self):
        print("ConcreteClass: PrimitiveOperation2")

concrete = ConcreteClass()
concrete.template_method()
```

### Ruby Example

```ruby
class AbstractClass
  def primitive_operation1
    raise 'Abstract method'
  end

  def primitive_operation2
    raise 'Abstract method'
  end

  def template_method
    primitive_operation1
    primitive_operation2
  end
end

class ConcreteClass < AbstractClass
  def primitive_operation1
    puts 'ConcreteClass: PrimitiveOperation1'
  end

  def primitive_operation2
    puts 'ConcreteClass: PrimitiveOperation2'
  end
end

concrete = ConcreteClass.new
concrete.template_method
```

### Rust Example

```rust
trait AbstractClass {
    fn primitive_operation1(&self);
    fn primitive_operation2(&self);

    fn template_method(&self) {
        self.primitive_operation1();
        self.primitive_operation2();
    }
}

struct ConcreteClass;

impl AbstractClass for ConcreteClass {
    fn primitive_operation1(&self) {
        println!("ConcreteClass: PrimitiveOperation1");
    }

    fn primitive_operation2(&self) {
        println!("ConcreteClass: PrimitiveOperation2");
    }
}

fn main() {
    let concrete = ConcreteClass;
    concrete.template_method();
}
```
