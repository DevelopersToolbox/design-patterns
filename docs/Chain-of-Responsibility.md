## Chain of Responsibility Pattern

The chain of responsibility pattern is a behavioural design pattern that allows an object to send a command without knowing which object will
handle it. This pattern creates a chain of processing objects, where each object in the chain can handle the request, pass it along the chain,
or decline to handle it. This approach promotes loose coupling between the sender and receivers of a request, enabling flexibility in assigning
responsibilities. The chain of responsibility pattern is particularly useful for scenarios where multiple objects might handle a request, and
the specific handler isn't known in advance.

### Go Example

```go
package main

import "fmt"

type Handler interface {
    Handle(request string)
}

type ConcreteHandler1 struct {
    successor Handler
}

func (h *ConcreteHandler1) Handle(request string) {
    if request == "one" {
        fmt.Println("ConcreteHandler1 handled the request")
    } else if h.successor != nil {
        h.successor.Handle(request)
    }
}

type ConcreteHandler2 struct {
    successor Handler
}

func (h *ConcreteHandler2) Handle(request string) {
    if request == "two" {
        fmt.Println("ConcreteHandler2 handled the request")
    } else if h.successor != nil {
        h.successor.Handle(request)
    }
}

func main() {
    handlerChain := &ConcreteHandler1{
        successor: &ConcreteHandler2{},
    }

    handlerChain.Handle("two")
    handlerChain.Handle("one")
}
```

### Perl Example

```perl
package Handler;
sub new {
    my ($class, $successor) = @_;
    return bless { successor => $successor }, $class;
}

sub handle {
    die "NotImplementedError";
}

package ConcreteHandler1;
use parent -norequire, 'Handler';

sub handle {
    my ($self, $request) = @_;
    if ($request eq 'one') {
        print "ConcreteHandler1 handled the request\n";
    } elsif ($self->{successor}) {
        $self->{successor}->handle($request);
    }
}

package ConcreteHandler2;
use parent -norequire, 'Handler';

sub handle {
    my ($self, $request) = @_;
    if ($request eq 'two') {
        print "ConcreteHandler2 handled the request\n";
    } elsif ($self->{successor}) {
        $self->{successor}->handle($request);
    }
}

my $handler_chain = ConcreteHandler1->new(ConcreteHandler2->new);
$handler_chain->handle('two');
$handler_chain->handle('one');
```

### Python Example

```python
class Handler:
    def __init__(self, successor=None):
        self.successor = successor

    def handle(self, request):
        raise NotImplementedError("Must provide implementation in subclass.")

class ConcreteHandler1(Handler):
    def handle(self, request):
        if request == "one":
            print("ConcreteHandler1 handled the request")
        elif self.successor:
            self.successor.handle(request)

class ConcreteHandler2(Handler):
    def handle(self, request):
        if request == "two":
            print("ConcreteHandler2 handled the request")
        elif self.successor:
            self.successor.handle(request)

handler_chain = ConcreteHandler1(ConcreteHandler2())
handler_chain.handle("two")
handler_chain.handle("one")
```

### Ruby Example

```ruby
class Handler
  def initialize(successor = nil)
    @successor = successor
  end

  def handle(request)
    raise 'NotImplementedError'
  end
end

class ConcreteHandler1 < Handler
  def handle(request)
    if request == 'one'
      puts 'ConcreteHandler1 handled the request'
    elsif @successor
      @successor.handle(request)
    end
  end
end

class ConcreteHandler2 < Handler
  def handle(request)
    if request == 'two'
      puts 'ConcreteHandler2 handled the request'
    elsif @successor
      @successor.handle(request)
    end
  end
end

handler_chain = ConcreteHandler1.new(ConcreteHandler2.new)
handler_chain.handle('two')
handler_chain.handle('one')
```

### Rust Example

```rust
trait Handler {
    fn handle(&self, request: &str);
}

struct ConcreteHandler1 {
    successor: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandler1 {
    fn handle(&self, request: &str) {
        if request == "one" {
            println!("ConcreteHandler1 handled the request");
        } else if let Some(ref successor) = self.successor {
            successor.handle(request);
        }
    }
}

struct ConcreteHandler2 {
    successor: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandler2 {
    fn handle(&self, request: &str) {
        if request == "two" {
            println!("ConcreteHandler2 handled the request");
        } else if let Some(ref successor) = self.successor {
            successor.handle(request);
        }
    }
}

fn main() {
    let handler_chain = ConcreteHandler1 {
        successor: Some(Box::new(ConcreteHandler2 { successor: None })),
    };

    handler_chain.handle("two");
    handler_chain.handle("one");
```
