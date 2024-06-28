## Strategy Pattern

The strategy pattern is a behavioural design pattern that defines a family of algorithms, encapsulates each one, and makes them interchangeable.
This pattern allows the algorithm to vary independently from the clients that use it, promoting flexibility and reusability. By defining a common
interface for all supported algorithms, the strategy pattern enables the client to choose the appropriate algorithm at runtime. This approach
enhances the extensibility of the system, allowing new algorithms to be added without modifying the existing code. The strategy pattern is
particularly useful for scenarios where multiple algorithms are applicable and the best one to use can vary based on context or user choice.

### Go Example

```go
package main

import "fmt"

// Strategy is the strategy interface
type Strategy interface {
	Execute(a, b int) int
}

// AddStrategy is a concrete strategy that adds two numbers
type AddStrategy struct{}

func (s AddStrategy) Execute(a, b int) int {
	return a + b
}

// SubtractStrategy is a concrete strategy that subtracts two numbers
type SubtractStrategy struct{}

func (s SubtractStrategy) Execute(a, b int) int {
	return a - b
}

// Context is the context that uses a strategy
type Context struct {
	strategy Strategy
}

func (c *Context) SetStrategy(strategy Strategy) {
	c.strategy = strategy
}

func (c *Context) ExecuteStrategy(a, b int) int {
	return c.strategy.Execute(a, b)
}

func main() {
	context := Context{strategy: AddStrategy{}}
	fmt.Println(context.ExecuteStrategy(5, 3))  // Output: 8

	context.SetStrategy(SubtractStrategy{})
	fmt.Println(context.ExecuteStrategy(5, 3))  // Output: 2
}
```

### Perl Example

```perl
package Strategy;
sub new {
    my ($class) = @_;
    return bless {}, $class;
}
sub execute {
    die "This method should be overridden";
}

package AddStrategy;
our @ISA = qw(Strategy);
sub execute {
    my ($self, $a, $b) = @_;
    return $a + $b;
}

package SubtractStrategy;
our @ISA = qw(Strategy);
sub execute {
    my ($self, $a, $b) = @_;
    return $a - $b;
}

package Context;
sub new {
    my ($class, $strategy) = @_;
    return bless { strategy => $strategy }, $class;
}
sub set_strategy {
    my ($self, $strategy) = @_;
    $self->{strategy} = $strategy;
}
sub execute_strategy {
    my ($self, $a, $b) = @_;
    return $self->{strategy}->execute($a, $b);
}

# Usage
my $context = Context->new(AddStrategy->new);
print $context->execute_strategy(5, 3), "\n";  # Output: 8

$context->set_strategy(SubtractStrategy->new);
print $context->execute_strategy(5, 3), "\n";  # Output: 2
```

### Python Example

```python
from abc import ABC, abstractmethod

class Strategy(ABC):
    @abstractmethod
    def execute(self, a, b):
        pass

class AddStrategy(Strategy):
    def execute(self, a, b):
        return a + b

class SubtractStrategy(Strategy):
    def execute(self, a, b):
        return a - b

class Context:
    def __init__(self, strategy: Strategy):
        self._strategy = strategy

    def set_strategy(self, strategy: Strategy):
        self._strategy = strategy

    def execute_strategy(self, a, b):
        return self._strategy.execute(a, b)

# Usage
context = Context(AddStrategy())
print(context.execute_strategy(5, 3))  # Output: 8

context.set_strategy(SubtractStrategy())
print(context.execute_strategy(5, 3))  # Output: 2
```

### Ruby Example

```ruby
class Strategy
  def execute(a, b)
    raise NotImplementedError, 'This method should be overridden'
  end
end

class AddStrategy < Strategy
  def execute(a, b)
    a + b
  end
end

class SubtractStrategy < Strategy
  def execute(a, b)
    a - b
  end
end

class Context
  def initialize(strategy)
    @strategy = strategy
  end

  def set_strategy(strategy)
    @strategy = strategy
  end

  def execute_strategy(a, b)
    @strategy.execute(a, b)
  end
end

# Usage
context = Context.new(AddStrategy.new)
puts context.execute_strategy(5, 3)  # Output: 8

context.set_strategy(SubtractStrategy.new)
puts context.execute_strategy(5, 3)  # Output: 2
```

### Rust Example

```rust
trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct AddStrategy;
impl Strategy for AddStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct SubtractStrategy;
impl Strategy for SubtractStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    fn execute_strategy(&self, a: i32, b: i32) -> i32 {
        self.strategy.execute(a, b)
    }
}

fn main() {
    let mut context = Context::new(Box::new(AddStrategy));
    println!("{}", context.execute_strategy(5, 3));  // Output: 8

    context.set_strategy(Box::new(SubtractStrategy));
    println!("{}", context.execute_strategy(5, 3));  // Output: 2
}
```
