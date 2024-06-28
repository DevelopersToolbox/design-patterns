## Memento Pattern

The memento pattern is a behavioural design pattern that captures and externalizes an object's internal state without violating encapsulation,
allowing the object to be restored to this state later. This pattern is useful for implementing undo/redo functionality. It involves creating
a memento object that stores the state of the originator object and providing methods to save and restore the state. The memento pattern ensures
that the object's state can be rolled back or restored without exposing its internal structure.

### Go Example

```go
package main

import "fmt"

// Memento
type Memento struct {
	state string
}

// Originator
type Originator struct {
	state string
}

func (o *Originator) SetState(state string) {
	o.state = state
}

func (o *Originator) SaveStateToMemento() *Memento {
	return &Memento{state: o.state}
}

func (o *Originator) GetStateFromMemento(memento *Memento) {
	o.state = memento.state
}

func (o *Originator) GetState() string {
	return o.state
}

// Caretaker
type Caretaker struct {
	mementoList []*Memento
}

func (c *Caretaker) Add(memento *Memento) {
	c.mementoList = append(c.mementoList, memento)
}

func (c *Caretaker) Get(index int) *Memento {
	return c.mementoList[index]
}

func main() {
	originator := &Originator{}
	caretaker := &Caretaker{}

	originator.SetState("State1")
	originator.SetState("State2")
	caretaker.Add(originator.SaveStateToMemento())

	originator.SetState("State3")
	caretaker.Add(originator.SaveStateToMemento())

	originator.SetState("State4")
	fmt.Println("Current State: " + originator.GetState())

	originator.GetStateFromMemento(caretaker.Get(0))
	fmt.Println("First saved State: " + originator.GetState())
	originator.GetStateFromMemento(caretaker.Get(1))
	fmt.Println("Second saved State: " + originator.GetState())
}
```

### Perl Example

```perl
package Memento;

sub new {
    my ($class, $state) = @_;
    return bless { state => $state }, $class;
}

sub get_state {
    my $self = shift;
    return $self->{state};
}

package Originator;

sub new {
    my $class = shift;
    return bless { state => '' }, $class;
}

sub set_state {
    my ($self, $state) = @_;
    $self->{state} = $state;
}

sub save_state_to_memento {
    my $self = shift;
    return Memento->new($self->{state});
}

sub get_state_from_memento {
    my ($self, $memento) = @_;
    $self->{state} = $memento->get_state();
}

sub get_state {
    my $self = shift;
    return $self->{state};
}

package Caretaker;

sub new {
    my $class = shift;
    return bless { memento_list => [] }, $class;
}

sub add {
    my ($self, $memento) = @_;
    push @{$self->{memento_list}}, $memento;
}

sub get {
    my ($self, $index) = @_;
    return $self->{memento_list}->[$index];
}

package main;

my $originator = Originator->new();
my $caretaker = Caretaker->new();

$originator->set_state("State1");
$originator->set_state("State2");
$caretaker->add($originator->save_state_to_memento());

$originator->set_state("State3");
$caretaker->add($originator->save_state_to_memento());

$originator->set_state("State4");
print "Current State: ", $originator->get_state(), "\n";

$originator->get_state_from_memento($caretaker->get(0));
print "First saved State: ", $originator->get_state(), "\n";
$originator->get_state_from_memento($caretaker->get(1));
print "Second saved State: ", $originator->get_state(), "\n";
```

### Python Example

```python
class Memento:
    def __init__(self, state):
        self.state = state

class Originator:
    def __init__(self):
        self.state = ''

    def set_state(self, state):
        self.state = state

    def save_state_to_memento(self):
        return Memento(self.state)

    def get_state_from_memento(self, memento):
        self.state = memento.state

    def get_state(self):
        return self.state

class Caretaker:
    def __init__(self):
        self.memento_list = []

    def add(self, memento):
        self.memento_list.append(memento)

    def get(self, index):
        return self.memento_list[index]

originator = Originator()
caretaker = Caretaker()

originator.set_state("State1")
originator.set_state("State2")
caretaker.add(originator.save_state_to_memento())

originator.set_state("State3")
caretaker.add(originator.save_state_to_memento())

originator.set_state("State4")
print("Current State:", originator.get_state())

originator.get_state_from_memento(caretaker.get(0))
print("First saved State:", originator.get_state())
originator.get_state_from_memento(caretaker.get(1))
print("Second saved State:", originator.get_state())
```

### Ruby Example

```ruby
class Memento
  attr_reader :state

  def initialize(state)
    @state = state
  end
end

class Originator
  attr_accessor :state

  def save_state_to_memento
    Memento.new(@state)
  end

  def get_state_from_memento(memento)
    @state = memento.state
  end
end

class Caretaker
  def initialize
    @memento_list = []
  end

  def add(memento)
    @memento_list << memento
  end

  def get(index)
    @memento_list[index]
  end
end

originator = Originator.new
caretaker = Caretaker.new

originator.state = "State1"
originator.state = "State2"
caretaker.add(originator.save_state_to_memento)

originator.state = "State3"
caretaker.add(originator.save_state_to_memento)

originator.state = "State4"
puts "Current State: #{originator.state}"

originator.get_state_from_memento(caretaker.get(0))
puts "First saved State: #{originator.state}"
originator.get_state_from_memento(caretaker.get(1))
puts "Second saved State: #{originator.state}"
```

### Rust Example

```rust
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Self {
        Memento { state }
    }

    fn get_state(&self) -> &str {
        &self.state
    }
}

struct Originator {
    state: String,
}

impl Originator {
    fn new() -> Self {
        Originator {
            state: String::new(),
        }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn save_state_to_memento(&self) -> Memento {
        Memento::new(self.state.clone())
    }

    fn get_state_from_memento(&mut self, memento: &Memento) {
        self.state = memento.get_state().to_string();
    }

    fn get_state(&self) -> &str {
        &self.state
    }
}

struct Caretaker {
    memento_list: Vec<Memento>,
}

impl Caretaker {
    fn new() -> Self {
        Caretaker {
            memento_list: Vec::new(),
        }
    }

    fn add(&mut self, memento: Memento) {
        self.memento_list.push(memento);
    }

    fn get(&self, index: usize) -> &Memento {
        &self.memento_list[index]
    }
}

fn main() {
    let mut originator = Originator::new();
    let mut caretaker = Caretaker::new();

    originator.set_state("State1".to_string());
    originator.set_state("State2".to_string());
    caretaker.add(originator.save_state_to_memento());

    originator.set_state("State3".to_string());
    caretaker.add(originator.save_state_to_memento());

    originator.set_state("State4".to_string());
    println!("Current State: {}", originator.get_state());

    originator.get_state_from_memento(caretaker.get(0));
    println!("First saved State: {}", originator.get_state());
    originator.get_state_from_memento(caretaker.get(1));
    println!("Second saved State: {}", originator.get_state());
}
```
