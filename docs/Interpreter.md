## Interpreter Pattern

The interpreter pattern is a behavioural design pattern that defines a grammatical representation for a language and provides an interpreter
to interpret sentences in the language. This pattern is useful for designing a language interpreter or compiler. It involves defining a
class hierarchy representing the grammar of the language and implementing an interpret method to process the language. The interpreter pattern
is particularly useful for simple languages or expressions that need to be evaluated dynamically.

### Go Example

```go
package main

import (
	"fmt"
	"strings"
)

// Expression
type Expression interface {
	Interpret(context string) bool
}

// TerminalExpression
type TerminalExpression struct {
	data string
}

func (t *TerminalExpression) Interpret(context string) bool {
	return strings.Contains(context, t.data)
}

// OrExpression
type OrExpression struct {
	expr1, expr2 Expression
}

func (o *OrExpression) Interpret(context string) bool {
	return o.expr1.Interpret(context) || o.expr2.Interpret(context)
}

// AndExpression
type AndExpression struct {
	expr1, expr2 Expression
}

func (a *AndExpression) Interpret(context string) bool {
	return a.expr1.Interpret(context) && a.expr2.Interpret(context)
}

func main() {
	robert := &TerminalExpression{data: "Robert"}
	john := &TerminalExpression{data: "John"}

	isSingle := &OrExpression{expr1: robert, expr2: john}

	julie := &TerminalExpression{data: "Julie"}
	married := &TerminalExpression{data: "Married"}

	isMarriedWoman := &AndExpression{expr1: julie, expr2: married}

	fmt.Println("John is single? ", isSingle.Interpret("John"))                 // Output: John is single? true
	fmt.Println("Julie is a married woman? ", isMarriedWoman.Interpret("Married Julie")) // Output: Julie is a married woman? true
}
```

### Perl Example

```perl
package Expression;

sub interpret {
    die "Abstract method";
}

package TerminalExpression;
use parent 'Expression';

sub new {
    my ($class, $data) = @_;
    return bless { data => $data }, $class;
}

sub interpret {
    my ($self, $context) = @_;
    return index($context, $self->{data}) != -1;
}

package OrExpression;
use parent 'Expression';

sub new {
    my ($class, $expr1, $expr2) = @_;
    return bless { expr1 => $expr1, expr2 => $expr2 }, $class;
}

sub interpret {
    my ($self, $context) = @_;
    return $self->{expr1}->interpret($context) || $self->{expr2}->interpret($context);
}

package AndExpression;
use parent 'Expression';

sub new {
    my ($class, $expr1, $expr2) = @_;
    return bless { expr1 => $expr1, expr2 => $expr2 }, $class;
}

sub interpret {
    my ($self, $context) = @_;
    return $self->{expr1}->interpret($context) && $self->{expr2}->interpret($context);
}

package main;

my $robert = TerminalExpression->new("Robert");
my $john = TerminalExpression->new("John");

my $is_single = OrExpression->new($robert, $john);

my $julie = TerminalExpression->new("Julie");
my $married = TerminalExpression->new("Married");

my $is_married_woman = AndExpression->new($julie, $married);

print "John is single? ", $is_single->interpret("John") ? "true" : "false", "\n"; # Output: John is single? true
print "Julie is a married woman? ", $is_married_woman->interpret("Married Julie") ? "true" : "false", "\n"; # Output: Julie is a married woman? true
```

### Python Example

```python
class Expression:
    def interpret(self, context):
        pass

class TerminalExpression(Expression):
    def __init__(self, data):
        self.data = data

    def interpret(self, context):
        return self.data in context

class OrExpression(Expression):
    def __init__(self, expr1, expr2):
        self.expr1 = expr1
        self.expr2 = expr2

    def interpret(self, context):
        return self.expr1.interpret(context) or self.expr2.interpret(context)

class AndExpression(Expression):
    def __init__(self, expr1, expr2):
        self.expr1 = expr1
        self.expr2 = expr2

    def interpret(self, context):
        return self.expr1.interpret(context) and self.expr2.interpret(context)

robert = TerminalExpression("Robert")
john = TerminalExpression("John")

is_single = OrExpression(robert, john)

julie = TerminalExpression("Julie")
married = TerminalExpression("Married")

is_married_woman = AndExpression(julie, married)

print("John is single? ", is_single.interpret("John"))                 # Output: John is single? true
print("Julie is a married woman? ", is_married_woman.interpret("Married Julie")) # Output: Julie is a married woman? true
```

### Ruby Example

```ruby
class Expression
  def interpret(context)
    raise 'Abstract method'
  end
end

class TerminalExpression < Expression
  def initialize(data)
    @data = data
  end

  def interpret(context)
    context.include?(@data)
  end
end

class OrExpression < Expression
  def initialize(expr1, expr2)
    @expr1 = expr1
    @expr2 = expr2
  end

  def interpret(context)
    @expr1.interpret(context) || @expr2.interpret(context)
  end
end

class AndExpression < Expression
  def initialize(expr1, expr2)
    @expr1 = expr1
    @expr2 = expr2
  end

  def interpret(context)
    @expr1.interpret(context) && @expr2.interpret(context)
  end
end

robert = TerminalExpression.new("Robert")
john = TerminalExpression.new("John")

is_single = OrExpression.new(robert, john)

julie = TerminalExpression.new("Julie")
married = TerminalExpression.new("Married")

is_married_woman = AndExpression.new(julie, married)

puts "John is single? #{is_single.interpret("John")}"                 # Output: John is single? true
puts "Julie is a married woman? #{is_married_woman.interpret("Married Julie")}" # Output: Julie is a married woman? true
```

### Rust Example

```rust
trait Expression {
    fn interpret(&self, context: &str) -> bool;
}

struct TerminalExpression {
    data: String,
}

impl TerminalExpression {
    fn new(data: &str) -> Self {
        TerminalExpression {
            data: data.to_string(),
        }
    }
}

impl Expression for TerminalExpression {
    fn interpret(&self, context: &str) -> bool {
        context.contains(&self.data)
    }
}

struct OrExpression {
    expr1: Box<dyn Expression>,
    expr2: Box<dyn Expression>,
}

impl OrExpression {
    fn new(expr1: Box<dyn Expression>, expr2: Box<dyn Expression>) -> Self {
        OrExpression { expr1, expr2 }
    }
}

impl Expression for OrExpression {
    fn interpret(&self, context: &str) -> bool {
        self.expr1.interpret(context) || self.expr2.interpret(context)
    }
}

struct AndExpression {
    expr1: Box<dyn Expression>,
    expr2: Box<dyn Expression>,
}

impl AndExpression {
    fn new(expr1: Box<dyn Expression>, expr2: Box<dyn Expression>) -> Self {
        AndExpression { expr1, expr2 }
    }
}

impl Expression for AndExpression {
    fn interpret(&self, context: &str) -> bool {
        self.expr1.interpret(context) && self.expr2.interpret(context)
    }
}

fn main() {
    let robert = Box::new(TerminalExpression::new("Robert"));
    let john = Box::new(TerminalExpression::new("John"));

    let is_single = OrExpression::new(robert, john);

    let julie = Box::new(TerminalExpression::new("Julie"));
    let married = Box::new(TerminalExpression::new("Married"));

    let is_married_woman = AndExpression::new(julie, married);

    println!("John is single? {}", is_single.interpret("John"));                 // Output: John is single? true
    println!("Julie is a married woman? {}", is_married_woman.interpret("Married Julie")); // Output: Julie is a married woman? true
}
```
