## Decorator Pattern

The decorator pattern is a structural design pattern that allows behaviour to be added to individual objects dynamically, without affecting the
behaviour of other objects from the same class. It provides a flexible alternative to subclassing for extending functionality. In this pattern,
a decorator class wraps the original class, adding new behaviour while maintaining the original class's interface. This approach promotes the
open/closed principle, allowing classes to be open for extension but closed for modification. The decorator pattern is particularly useful for
adding features to objects in a flexible and reusable manner.


### Go Example

```go
package main

import "fmt"

type Coffee interface {
    Cost() float64
}

type BasicCoffee struct{}

func (c BasicCoffee) Cost() float64 {
    return 5.0
}

type MilkDecorator struct {
    coffee Coffee
}

func (m MilkDecorator) Cost() float64 {
    return m.coffee.Cost() + 1.0
}

type SugarDecorator struct {
    coffee Coffee
}

func (s SugarDecorator) Cost() float64 {
    return s.coffee.Cost() + 0.5
}

func main() {
    coffee := BasicCoffee{}
    fmt.Println(coffee.Cost()) // 5.0

    coffeeWithMilk := MilkDecorator{coffee: coffee}
    fmt.Println(coffeeWithMilk.Cost()) // 6.0

    coffeeWithMilkAndSugar := SugarDecorator{coffee: coffeeWithMilk}
    fmt.Println(coffeeWithMilkAndSugar.Cost()) // 6.5
}
```

### Perl Example

```perl
package Coffee;
sub new { bless {}, shift }
sub cost { 5 }

package MilkDecorator;
sub new { my ($class, $coffee) = @_; bless { coffee => $coffee }, $class }
sub cost { my $self = shift; $self->{coffee}->cost + 1 }

package SugarDecorator;
sub new { my ($class, $coffee) = @_; bless { coffee => $coffee }, $class }
sub cost { my $self = shift; $self->{coffee}->cost + 0.5 }

# Usage
my $coffee = Coffee->new;
print $coffee->cost . "\n";  # 5

my $coffee_with_milk = MilkDecorator->new($coffee);
print $coffee_with_milk->cost . "\n";  # 6

my $coffee_with_milk_and_sugar = SugarDecorator->new($coffee_with_milk);
print $coffee_with_milk_and_sugar->cost . "\n";  # 6.5
```

### Python Example

```python
class Coffee:
    def cost(self):
        return 5

class MilkDecorator:
    def __init__(self, coffee):
        self._coffee = coffee

    def cost(self):
        return self._coffee.cost() + 1

class SugarDecorator:
    def __init__(self, coffee):
        self._coffee = coffee

    def cost(self):
        return self._coffee.cost() + 0.5

# Usage
coffee = Coffee()
print(coffee.cost())  # 5

coffee_with_milk = MilkDecorator(coffee)
print(coffee_with_milk.cost())  # 6

coffee_with_milk_and_sugar = SugarDecorator(coffee_with_milk)
print(coffee_with_milk_and_sugar.cost())  # 6.5
```

### Ruby Example

```ruby
class Coffee
  def cost
    5
  end
end

class MilkDecorator
  def initialize(coffee)
    @coffee = coffee
  end

  def cost
    @coffee.cost + 1
  end
end

class SugarDecorator
  def initialize(coffee)
    @coffee = coffee
  end

  def cost
    @coffee.cost + 0.5
  end
end

# Usage
coffee = Coffee.new
puts coffee.cost  # 5

coffee_with_milk = MilkDecorator.new(coffee)
puts coffee_with_milk.cost  # 6

coffee_with_milk_and_sugar = SugarDecorator.new(coffee_with_milk)
puts coffee_with_milk_and_sugar.cost  # 6.5
```

### Rust Example

```rust
trait Coffee {
    fn cost(&self) -> f64;
}

struct BasicCoffee;

impl Coffee for BasicCoffee {
    fn cost(&self) -> f64 {
        5.0
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> MilkDecorator {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 1.0
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> SugarDecorator {
        SugarDecorator { coffee }
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }
}

fn main() {
    let coffee = BasicCoffee;
    println!("{}", coffee.cost()); // 5.0

    let coffee_with_milk = MilkDecorator::new(Box::new(coffee));
    println!("{}", coffee_with_milk.cost()); // 6.0

    let coffee_with_milk_and_sugar = SugarDecorator::new(Box::new(coffee_with_milk));
    println!("{}", coffee_with_milk_and_sugar.cost()); // 6.5
}
```
