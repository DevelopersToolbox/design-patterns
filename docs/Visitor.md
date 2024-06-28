## Visitor Pattern

The visitor pattern is a behavioural design pattern that separates an algorithm from the objects on which it operates. This pattern involves
creating a visitor class that implements operations to be performed on elements of an object structure. By accepting a visitor, each element
allows the visitor to perform the operation, decoupling the algorithm from the object structure. The visitor pattern makes it easy to add new
operations without modifying the objects, promoting flexibility and enhancing maintainability. It is particularly useful for operations that
need to be performed on a variety of objects with different interfaces.


### Go Example

```go
package main

import "fmt"

// Element
type Element interface {
	Accept(visitor Visitor)
}

// ConcreteElementA
type ConcreteElementA struct{}

func (e *ConcreteElementA) Accept(visitor Visitor) {
	visitor.VisitConcreteElementA(e)
}

// ConcreteElementB
type ConcreteElementB struct{}

func (e *ConcreteElementB) Accept(visitor Visitor) {
	visitor.VisitConcreteElementB(e)
}

// Visitor
type Visitor interface {
	VisitConcreteElementA(element *ConcreteElementA)
	VisitConcreteElementB(element *ConcreteElementB)
}

// ConcreteVisitor
type ConcreteVisitor struct{}

func (v *ConcreteVisitor) VisitConcreteElementA(element *ConcreteElementA) {
	fmt.Println("Visited ConcreteElementA")
}

func (v *ConcreteVisitor) VisitConcreteElementB(element *ConcreteElementB) {
	fmt.Println("Visited ConcreteElementB")
}

func main() {
	elements := []Element{&ConcreteElementA{}, &ConcreteElementB{}}
	visitor := &ConcreteVisitor{}

	for _, element := range elements {
		element.Accept(visitor)
	}
}
```

### Perl Example

```perl
package Element;

sub accept {
    die "Abstract method";
}

package ConcreteElementA;
use parent 'Element';

sub accept {
    my ($self, $visitor) = @_;
    $visitor->visit_concrete_element_a($self);
}

package ConcreteElementB;
use parent 'Element';

sub accept {
    my ($self, $visitor) = @_;
    $visitor->visit_concrete_element_b($self);
}

package Visitor;

sub visit_concrete_element_a { die "Abstract method" }
sub visit_concrete_element_b { die "Abstract method" }

package ConcreteVisitor;
use parent 'Visitor';

sub visit_concrete_element_a {
    print "Visited ConcreteElementA\n";
}

sub visit_concrete_element_b {
    print "Visited ConcreteElementB\n";
}

package main;

my @elements = (ConcreteElementA->new(), ConcreteElementB->new());
my $visitor = ConcreteVisitor->new();

for my $element (@elements) {
    $element->accept($visitor);
}
```

### Python Example

```python
class Element:
    def accept(self, visitor):
        pass

class ConcreteElementA(Element):
    def accept(self, visitor):
        visitor.visit_concrete_element_a(self)

class ConcreteElementB(Element):
    def accept(self, visitor):
        visitor.visit_concrete_element_b(self)

class Visitor:
    def visit_concrete_element_a(self, element):
        pass

    def visit_concrete_element_b(self, element):
        pass

class ConcreteVisitor(Visitor):
    def visit_concrete_element_a(self, element):
        print("Visited ConcreteElementA")

    def visit_concrete_element_b(self, element):
        print("Visited ConcreteElementB")

elements = [ConcreteElementA(), ConcreteElementB()]
visitor = ConcreteVisitor()

for element in elements:
    element.accept(visitor)
```

### Ruby Example

```ruby
class Element
  def accept(visitor)
    raise 'Abstract method'
  end
end

class ConcreteElementA < Element
  def accept(visitor)
    visitor.visit_concrete_element_a(self)
  end
end

class ConcreteElementB < Element
  def accept(visitor)
    visitor.visit_concrete_element_b(self)
  end
end

class Visitor
  def visit_concrete_element_a(element)
    raise 'Abstract method'
  end

  def visit_concrete_element_b(element)
    raise 'Abstract method'
  end
end

class ConcreteVisitor < Visitor
  def visit_concrete_element_a(element)
    puts 'Visited ConcreteElementA'
  end

  def visit_concrete_element_b(element)
    puts 'Visited ConcreteElementB'
  end
end

elements = [ConcreteElementA.new, ConcreteElementB.new]
visitor = ConcreteVisitor.new

elements.each do |element|
  element.accept(visitor)
end
```

### Rust Example

```rust
trait Visitor {
    fn visit_concrete_element_a(&self, element: &ConcreteElementA);
    fn visit_concrete_element_b(&self, element: &ConcreteElementB);
}

struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_concrete_element_a(&self, _element: &ConcreteElementA) {
        println!("Visited ConcreteElementA");
    }

    fn visit_concrete_element_b(&self, _element: &ConcreteElementB) {
        println!("Visited ConcreteElementB");
    }
}

trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}

struct ConcreteElementA;

impl Element for ConcreteElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_concrete_element_a(self);
    }
}

struct ConcreteElementB;

impl Element for ConcreteElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_concrete_element_b(self);
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> = vec![Box::new(ConcreteElementA), Box::new(ConcreteElementB)];
    let visitor = ConcreteVisitor;

    for element in elements {
        element.accept(&visitor);
    }
}
```
