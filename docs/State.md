## State Pattern

The state pattern is a behavioural design pattern that allows an object to alter its behaviour when its internal state changes. This pattern
encapsulates state-specific behaviour within state objects, enabling the object to appear as if it changes its class when its state changes.
The state pattern promotes the open/closed principle, allowing new states to be added without modifying the existing code. It simplifies state
transitions and enhances code maintainability by organizing state-specific behaviour into separate classes.

### Go Example

```go
package main

import "fmt"

// State interface
type State interface {
	Handle(context *Context)
}

// Context struct
type Context struct {
	state State
}

func (c *Context) SetState(state State) {
	c.state = state
}

func (c *Context) Request() {
	c.state.Handle(c)
}

// ConcreteStateA struct
type ConcreteStateA struct{}

func (s *ConcreteStateA) Handle(context *Context) {
	fmt.Println("State A handling request and changing to State B")
	context.SetState(&ConcreteStateB{})
}

// ConcreteStateB struct
type ConcreteStateB struct{}

func (s *ConcreteStateB) Handle(context *Context) {
	fmt.Println("State B handling request and changing to State A")
	context.SetState(&ConcreteStateA{})
}

func main() {
	context := &Context{state: &ConcreteStateA{}}

	context.Request() // Output: State A handling request and changing to State B
	context.Request() // Output: State B handling request and changing to State A
	context.Request() // Output: State A handling request and changing to State B
	context.Request() // Output: State B handling request and changing to State A
}
```

### Perl Example

```perl
package State;

sub handle {
    die "Abstract method";
}

package Context;

sub new {
    my ($class, $state) = @_;
    return bless { state => $state }, $class;
}

sub set_state {
    my ($self, $state) = @_;
    $self->{state} = $state;
}

sub request {
    my $self = shift;
    $self->{state}->handle($self);
}

package ConcreteStateA;
use parent 'State';

sub handle {
    my ($self, $context) = @_;
    print "State A handling request and changing to State B\n";
    $context->set_state(ConcreteStateB->new());
}

package ConcreteStateB;
use parent 'State';

sub handle {
    my ($self, $context) = @_;
    print "State B handling request and changing to State A\n";
    $context->set_state(ConcreteStateA->new());
}

package main;

my $context = Context->new(ConcreteStateA->new());

$context->request(); # Output: State A handling request and changing to State B
$context->request(); # Output: State B handling request and changing to State A
$context->request(); # Output: State A handling request and changing to State B
$context->request(); # Output: State B handling request and changing to State A
```

### Python Example

```python
from abc import ABC, abstractmethod

class State(ABC):
    @abstractmethod
    def handle(self, context):
        pass

class Context:
    def __init__(self, state):
        self.state = state

    def set_state(self, state):
        self.state = state

    def request(self):
        self.state.handle(self)

class ConcreteStateA(State):
    def handle(self, context):
        print("State A handling request and changing to State B")
        context.set_state(ConcreteStateB())

class ConcreteStateB(State):
    def handle(self, context):
        print("State B handling request and changing to State A")
        context.set_state(ConcreteStateA())

context = Context(ConcreteStateA())

context.request()  # Output: State A handling request and changing to State B
context.request()  # Output: State B handling request and changing to State A
context.request()  # Output: State A handling request and changing to State B
context.request()  # Output: State B handling request and changing to State A
```

### Ruby Example

```ruby
class State
  def handle(context)
    raise 'Abstract method'
  end
end

class Context
  attr_accessor :state

  def initialize(state)
    @state = state
  end

  def request
    @state.handle(self)
  end
end

class ConcreteStateA < State
  def handle(context)
    puts 'State A handling request and changing to State B'
    context.state = ConcreteStateB.new
  end
end

class ConcreteStateB < State
  def handle(context)
    puts 'State B handling request and changing to State A'
    context.state = ConcreteStateA.new
  end
end

context = Context.new(ConcreteStateA.new)

context.request # Output: State A handling request and changing to State B
context.request # Output: State B handling request and changing to State A
context.request # Output: State A handling request and changing to State B
context.request # Output: State B handling request and changing to State A
```

### Rust Example

```rust
trait State {
    fn handle(&self, context: &mut Context);
}

struct Context {
    state: Box<dyn State>,
}

impl Context {
    fn new(state: Box<dyn State>) -> Self {
        Context { state }
    }

    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    fn request(&mut self) {
        self.state.handle(self);
    }
}

struct ConcreteStateA;

impl State for ConcreteStateA {
    fn handle(&self, context: &mut Context) {
        println!("State A handling request and changing to State B");
        context.set_state(Box::new(ConcreteStateB));
    }
}

struct ConcreteStateB;

impl State for ConcreteStateB {
    fn handle(&self, context: &mut Context) {
        println!("State B handling request and changing to State A");
        context.set_state(Box::new(ConcreteStateA));
    }
}

fn main() {
    let mut context = Context::new(Box::new(ConcreteStateA));

    context.request(); // Output: State A handling request and changing to State B
    context.request(); // Output: State B handling request and changing to State A
    context.request(); // Output: State A handling request and changing to State B
    context.request(); // Output: State B handling request and changing to State A
}
```
