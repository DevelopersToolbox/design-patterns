## Proxy Pattern

The proxy pattern is a structural design pattern that provides a surrogate or placeholder for another object to control access to it.
This pattern is used to add an extra level of indirection to support controlled access, lazy initialization, logging, and other similar
tasks. By implementing the same interface as the underlying object, the proxy can act as a stand-in, forwarding requests to the real object
while adding its behaviour. This approach enhances control over the underlying object, promoting flexibility and maintainability in software design.

### Go Example

```go
package main

import "fmt"

type Subject interface {
    Request() string
}

type RealSubject struct{}

func (r *RealSubject) Request() string {
    return "RealSubject: Handling request."
}

type Proxy struct {
    realSubject *RealSubject
}

func (p *Proxy) Request() string {
    if p.checkAccess() {
        result := p.realSubject.Request()
        p.logAccess()
        return result
    }
    return "Access denied."
}

func (p *Proxy) checkAccess() bool {
    fmt.Println("Proxy: Checking access prior to firing a real request.")
    return true
}

func (p *Proxy) logAccess() {
    fmt.Println("Proxy: Logging the time of request.")
}

func main() {
    realSubject := &RealSubject{}
    proxy := &Proxy{realSubject}
    fmt.Println(proxy.Request())
}
```

### Perl Example

```perl
package RealSubject;
sub new {
    my $class = shift;
    bless {}, $class;
}

sub request {
    return "RealSubject: Handling request.";
}

package Proxy;
sub new {
    my ($class, $real_subject) = @_;
    bless { real_subject => $real_subject }, $class;
}

sub request {
    my $self = shift;
    if ($self->check_access()) {
        my $result = $self->{real_subject}->request();
        $self->log_access();
        return $result;
    }
}

sub check_access {
    print "Proxy: Checking access prior to firing a real request.\n";
    return 1;
}

sub log_access {
    print "Proxy: Logging the time of request.\n";
}

package main;
my $real_subject = RealSubject->new();
my $proxy = Proxy->new($real_subject);
print $proxy->request() . "\n";
```

### Python Example

```python
class RealSubject:
    def request(self):
        return "RealSubject: Handling request."

class Proxy:
    def __init__(self, real_subject):
        self._real_subject = real_subject

    def request(self):
        if self.check_access():
            result = self._real_subject.request()
            self.log_access()
            return result

    def check_access(self):
        print("Proxy: Checking access prior to firing a real request.")
        return True

    def log_access(self):
        print("Proxy: Logging the time of request.")

if __name__ == "__main__":
    real_subject = RealSubject()
    proxy = Proxy(real_subject)
    print(proxy.request())
```

### Ruby Example

```ruby
class RealSubject
  def request
    "RealSubject: Handling request."
  end
end

class Proxy
  def initialize(real_subject)
    @real_subject = real_subject
  end

  def request
    if check_access
      result = @real_subject.request
      log_access
      result
    end
  end

  def check_access
    puts "Proxy: Checking access prior to firing a real request."
    true
  end

  def log_access
    puts "Proxy: Logging the time of request."
  end
end

real_subject = RealSubject.new
proxy = Proxy.new(real_subject)
puts proxy.request
```

### Rust Example

```rust
trait Subject {
    fn request(&self) -> String;
}

struct RealSubject;

impl Subject for RealSubject {
    fn request(&self) -> String {
        "RealSubject: Handling request.".to_string()
    }
}

struct Proxy {
    real_subject: RealSubject,
}

impl Proxy {
    fn new(real_subject: RealSubject) -> Self {
        Proxy { real_subject }
    }

    fn check_access(&self) -> bool {
        println!("Proxy: Checking access prior to firing a real request.");
        true
    }

    fn log_access(&self) {
        println!("Proxy: Logging the time of request.");
    }
}

impl Subject for Proxy {
    fn request(&self) -> String {
        if self.check_access() {
            let result = self.real_subject.request();
            self.log_access();
            result
        } else {
            "Access denied.".to_string()
        }
    }
}

fn main() {
    let real_subject = RealSubject;
    let proxy = Proxy::new(real_subject);
    println!("{}", proxy.request());
}
```
