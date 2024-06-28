## Abstract Factory Pattern

The abstract factory pattern is a creational design pattern that allows the creation of objects without specifying their exact class. It
provides an interface for creating families of related or dependent objects, ensuring that the created objects are compatible. This pattern
promotes consistency among products and is particularly useful when the system needs to be independent of how its products are created,
composed, and represented.

### Go Example

```go
package main

import "fmt"

// AbstractFactory
type ShoeFactory interface {
	MakeShoe() Shoe
}

// ConcreteFactory1
type NikeFactory struct{}

func (n *NikeFactory) MakeShoe() Shoe {
	return &NikeShoe{}
}

// ConcreteFactory2
type AdidasFactory struct{}

func (a *AdidasFactory) MakeShoe() Shoe {
	return &AdidasShoe{}
}

// AbstractProduct
type Shoe interface {
	GetLogo() string
}

// ConcreteProduct1
type NikeShoe struct{}

func (n *NikeShoe) GetLogo() string {
	return "Nike"
}

// ConcreteProduct2
type AdidasShoe struct{}

func (a *AdidasShoe) GetLogo() string {
	return "Adidas"
}

func main() {
	nikeFactory := &NikeFactory{}
	adidasFactory := &AdidasFactory{}

	nikeShoe := nikeFactory.MakeShoe()
	adidasShoe := adidasFactory.MakeShoe()

	fmt.Println(nikeShoe.GetLogo())  // Output: Nike
	fmt.Println(adidasShoe.GetLogo()) // Output: Adidas
}
```

### Perl Example

```perl
package ShoeFactory;

use strict;
use warnings;

sub make_shoe { die "Abstract method" }

package NikeFactory;
use parent 'ShoeFactory';

sub make_shoe {
    return NikeShoe->new();
}

package AdidasFactory;
use parent 'ShoeFactory';

sub make_shoe {
    return AdidasShoe->new();
}

package Shoe;

sub get_logo { die "Abstract method" }

package NikeShoe;
use parent 'Shoe';

sub new { bless {}, shift }

sub get_logo { return "Nike" }

package AdidasShoe;
use parent 'Shoe';

sub new { bless {}, shift }

sub get_logo { return "Adidas" }

package main;

my $nike_factory = NikeFactory->new();
my $adidas_factory = AdidasFactory->new();

my $nike_shoe = $nike_factory->make_shoe();
my $adidas_shoe = $adidas_factory->make_shoe();

print $nike_shoe->get_logo(), "\n";  # Output: Nike
print $adidas_shoe->get_logo(), "\n"; # Output: Adidas
```

### Python Example

```python
from abc import ABC, abstractmethod

class Shoe(ABC):
    @abstractmethod
    def get_logo(self):
        pass

class NikeShoe(Shoe):
    def get_logo(self):
        return "Nike"

class AdidasShoe(Shoe):
    def get_logo(self):
        return "Adidas"

class ShoeFactory(ABC):
    @abstractmethod
    def make_shoe(self):
        pass

class NikeFactory(ShoeFactory):
    def make_shoe(self):
        return NikeShoe()

class AdidasFactory(ShoeFactory):
    def make_shoe(self):
        return AdidasShoe()

nike_factory = NikeFactory()
adidas_factory = AdidasFactory()

nike_shoe = nike_factory.make_shoe()
adidas_shoe = adidas_factory.make_shoe()

print(nike_shoe.get_logo())  # Output: Nike
print(adidas_shoe.get_logo()) # Output: Adidas
```

### Ruby Example

```ruby
class Shoe
  def get_logo
    raise 'Abstract method'
  end
end

class NikeShoe < Shoe
  def get_logo
    'Nike'
  end
end

class AdidasShoe < Shoe
  def get_logo
    'Adidas'
  end
end

class ShoeFactory
  def make_shoe
    raise 'Abstract method'
  end
end

class NikeFactory < ShoeFactory
  def make_shoe
    NikeShoe.new
  end
end

class AdidasFactory < ShoeFactory
  def make_shoe
    AdidasShoe.new
  end
end

nike_factory = NikeFactory.new
adidas_factory = AdidasFactory.new

nike_shoe = nike_factory.make_shoe
adidas_shoe = adidas_factory.make_shoe

puts nike_shoe.get_logo  # Output: Nike
puts adidas_shoe.get_logo # Output: Adidas
```

### Rust Example

```rust
trait Shoe {
    fn get_logo(&self) -> &str;
}

struct NikeShoe;

impl Shoe for NikeShoe {
    fn get_logo(&self) -> &str {
        "Nike"
    }
}

struct AdidasShoe;

impl Shoe for AdidasShoe {
    fn get_logo(&self) -> &str {
        "Adidas"
    }
}

trait ShoeFactory {
    fn make_shoe(&self) -> Box<dyn Shoe>;
}

struct NikeFactory;

impl ShoeFactory for NikeFactory {
    fn make_shoe(&self) -> Box<dyn Shoe> {
        Box::new(NikeShoe)
    }
}

struct AdidasFactory;

impl ShoeFactory for AdidasFactory {
    fn make_shoe(&self) -> Box<dyn Shoe> {
        Box::new(AdidasShoe)
    }
}

fn main() {
    let nike_factory = NikeFactory;
    let adidas_factory = AdidasFactory;

    let nike_shoe = nike_factory.make_shoe();
    let adidas_shoe = adidas_factory.make_shoe();

    println!("{}", nike_shoe.get_logo());  // Output: Nike
    println!("{}", adidas_shoe.get_logo()); // Output: Adidas
}
```
