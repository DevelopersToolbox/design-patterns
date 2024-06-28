## Observer Pattern

The observer pattern is a behavioural design pattern that defines a one-to-many dependency between objects, ensuring that when one object
changes state, all its dependents are notified and updated automatically. This pattern is commonly used in event-driven programming to
implement distributed event-handling systems. By promoting loose coupling between the subject (the object being observed) and the observers
(the objects watching for changes), the observer pattern enhances flexibility and reusability. It allows for dynamic subscription and notification,
making it easier to manage and coordinate the behaviour of multiple objects in response to state changes.

### Go Example

```go
package main

import "fmt"

type Observer interface {
    Update(message string)
}

type Subject struct {
    observers []Observer
}

func (s *Subject) Attach(observer Observer) {
    s.observers = append(s.observers, observer)
}

func (s *Subject) Detach(observer Observer) {
    for i, o := range s.observers {
        if o == observer {
            s.observers = append(s.observers[:i], s.observers[i+1:]...)
            break
        }
    }
}

func (s *Subject) Notify(message string) {
    for _, observer := range s.observers {
        observer.Update(message)
    }
}

type ConcreteObserver struct{}

func (co *ConcreteObserver) Update(message string) {
    fmt.Println("Received message:", message)
}

func main() {
    subject := &Subject{}
    observer := &ConcreteObserver{}

    subject.Attach(observer)
    subject.Notify("Hello, World!")
}
```

### Perl Example

```perl
package Subject;
sub new {
    my $class = shift;
    my $self = { observers => [] };
    bless $self, $class;
    return $self;
}

sub attach {
    my ($self, $observer) = @_;
    push @{$self->{observers}}, $observer;
}

sub detach {
    my ($self, $observer) = @_;
    @{$self->{observers}} = grep { $_ != $observer } @{$self->{observers}};
}

sub notify {
    my ($self, $message) = @_;
    $_->update($message) for @{$self->{observers}};
}

package Observer;
sub update {
    my ($self, $message) = @_;
    # To be implemented by concrete observers
}

package ConcreteObserver;
use base 'Observer';

sub update {
    my ($self, $message) = @_;
    print "Received message: $message\n";
}

# Usage
my $subject = Subject->new;
my $observer = ConcreteObserver->new;

$subject->attach($observer);
$subject->notify("Hello, World!");
```

### Python Example

```python
class Subject:
    def __init__(self):
        self._observers = []

    def attach(self, observer):
        self._observers.append(observer)

    def detach(self, observer):
        self._observers.remove(observer)

    def notify(self, message):
        for observer in self._observers:
            observer.update(message)

class Observer:
    def update(self, message):
        pass

class ConcreteObserver(Observer):
    def update(self, message):
        print(f"Received message: {message}")

# Usage
subject = Subject()
observer = ConcreteObserver()

subject.attach(observer)
subject.notify("Hello, World!")
```

### Ruby Example

```ruby
class Subject
  def initialize
    @observers = []
  end

  def attach(observer)
    @observers << observer
  end

  def detach(observer)
    @observers.delete(observer)
  end

  def notify(message)
    @observers.each { |observer| observer.update(message) }
  end
end

class Observer
  def update(message)
    # To be implemented by concrete observers
  end
end

class ConcreteObserver < Observer
  def update(message)
    puts "Received message: #{message}"
  end
end

# Usage
subject = Subject.new
observer = ConcreteObserver.new

subject.attach(observer)
subject.notify("Hello, World!")
```

### Rust Example

```rust
trait Observer {
    fn update(&self, message: &str);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, index: usize) {
        self.observers.remove(index);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

struct ConcreteObserver;

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("Received message: {}", message);
    }
}

fn main() {
    let mut subject = Subject::new();
    let observer = Box::new(ConcreteObserver);

    subject.attach(observer);
    subject.notify("Hello, World!");```
