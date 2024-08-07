## Adapter Pattern

The adapter pattern is a structural design pattern that enables objects with incompatible interfaces to collaborate. It acts as a bridge,
converting the interface of a class into one that a client expects. This pattern is particularly useful when integrating new components into
an existing system, as it allows for the reuse of existing functionalities without modifying their source code. By wrapping an existing class
with a new interface, the adapter pattern ensures compatibility and seamless integration, promoting flexibility and extensibility in software design.

### Go Example

```go
package main

import "fmt"

// Target is the interface that the client expects.
type Target interface {
	Request() string
}

// Adaptee contains some useful behavior, but its interface is incompatible
// with the existing client code.
type Adaptee struct{}

// SpecificRequest is the incompatible method of Adaptee.
func (a *Adaptee) SpecificRequest() string {
	return ".eetpadA eht fo roivaheb laicepS"
}

// Adapter makes Adaptee's interface compatible with the Target interface.
type Adapter struct {
	adaptee *Adaptee
}

// NewAdapter constructs a new Adapter.
func NewAdapter(adaptee *Adaptee) *Adapter {
	return &Adapter{adaptee}
}

// Request calls the incompatible method and adapts the result.
func (a *Adapter) Request() string {
	runes := []rune(a.adaptee.SpecificRequest())
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return fmt.Sprintf("Adapter: (TRANSLATED) %s", string(runes))
}

func clientCode(target Target) {
	fmt.Println(target.Request())
}

func main() {
	adaptee := &Adaptee{}
	adapter := NewAdapter(adaptee)
	clientCode(adapter)
}
```

### Perl Example

```perl
package Target;
sub new { bless {}, shift }
sub request { "Target: The default target's behavior." }

package Adaptee;
sub new { bless {}, shift }
sub specific_request { ".eetpadA eht fo roivaheb laicepS" }

package Adapter;
sub new {
    my ($class, $adaptee) = @_;
    bless { adaptee => $adaptee }, $class;
}

sub request {
    my $self = shift;
    my $result = reverse $self->{adaptee}->specific_request();
    return "Adapter: (TRANSLATED) $result";
}

package main;
sub client_code {
    my $target = shift;
    print $target->request, "\n";
}

my $target = Target->new;
client_code($target);

my $adaptee = Adaptee->new;
my $adapter = Adapter->new($adaptee);
client_code($adapter);
```

### Python Example

```python
class Target:
    def request(self):
        return "Target: The default target's behavior."


class Adaptee:
    def specific_request(self):
        return ".eetpadA eht fo roivaheb laicepS"


class Adapter(Target):
    def __init__(self, adaptee):
        self.adaptee = adaptee

    def request(self):
        return f"Adapter: (TRANSLATED) {self.adaptee.specific_request()[::-1]}"


def client_code(target):
    print(target.request())


if __name__ == "__main__":
    target = Target()
    client_code(target)

    adaptee = Adaptee()
    adapter = Adapter(adaptee)
    client_code(adapter)
```

### Ruby Example

```ruby
class Target
  def request
    "Target: The default target's behavior."
  end
end

class Adaptee
  def specific_request
    ".eetpadA eht fo roivaheb laicepS"
  end
end

class Adapter < Target
  def initialize(adaptee)
    @adaptee = adaptee
  end

  def request
    "Adapter: (TRANSLATED) #{reverse_string(@adaptee.specific_request)}"
  end

  def reverse_string(str)
    str.reverse
  end
end

def client_code(target)
  puts target.request
end

target = Target.new
client_code(target)

adaptee = Adaptee.new
adapter = Adapter.new(adaptee)
client_code(adapter)
```

### Rust Example

```rust
trait Target {
    fn request(&self) -> String;
}

struct Adaptee;

impl Adaptee {
    fn specific_request(&self) -> String {
        ".eetpadA eht fo roivaheb laicepS".to_string()
    }
}

struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        format!("Adapter: (TRANSLATED) {}", self.adaptee.specific_request().chars().rev().collect::<String>())
    }
}

fn client_code(target: &dyn Target) {
    println!("{}", target.request());
}

fn main() {
    let adaptee = Adaptee;
    let adapter = Adapter::new(adaptee);
    client_code(&adapter);
}
```
