## Mediator Pattern

The mediator pattern is a behavioural design pattern that defines an object, known as the mediator, which encapsulates how a set of objects
interact. This pattern promotes loose coupling by preventing objects from referring to each other explicitly. Instead, they communicate
through the mediator, which controls the interactions. The mediator pattern simplifies the maintenance and modification of interactions
between objects, making it easier to manage complex dependencies and enhance reusability.

### Go Example

```go
package main

import "fmt"

// Mediator
type Mediator interface {
	ShowMessage(user *User, message string)
}

// ConcreteMediator
type ChatRoom struct{}

func (c *ChatRoom) ShowMessage(user *User, message string) {
	fmt.Printf("%s: %s\n", user.GetName(), message)
}

// Colleague
type User struct {
	name     string
	mediator Mediator
}

func (u *User) GetName() string {
	return u.name
}

func (u *User) SendMessage(message string) {
	u.mediator.ShowMessage(u, message)
}

func main() {
	chatRoom := &ChatRoom{}

	user1 := &User{name: "Alice", mediator: chatRoom}
	user2 := &User{name: "Bob", mediator: chatRoom}

	user1.SendMessage("Hi, Bob!") // Output: Alice: Hi, Bob!
	user2.SendMessage("Hello, Alice!") // Output: Bob: Hello, Alice!
}
```

### Perl Example

```perl
package Mediator;

sub show_message {
    die "Abstract method";
}

package ChatRoom;
use parent 'Mediator';

sub show_message {
    my ($self, $user, $message) = @_;
    print $user->get_name(), ": ", $message, "\n";
}

package User;

sub new {
    my ($class, $name, $mediator) = @_;
    return bless { name => $name, mediator => $mediator }, $class;
}

sub get_name {
    my $self = shift;
    return $self->{name};
}

sub send_message {
    my ($self, $message) = @_;
    $self->{mediator}->show_message($self, $message);
}

package main;

my $chat_room = ChatRoom->new();

my $user1 = User->new("Alice", $chat_room);
my $user2 = User->new("Bob", $chat_room);

$user1->send_message("Hi, Bob!"); # Output: Alice: Hi, Bob!
$user2->send_message("Hello, Alice!"); # Output: Bob: Hello, Alice!
```

### Python Example

```python
class ChatRoom:
    @staticmethod
    def show_message(user, message):
        print(f"{user.name}: {message}")

class User:
    def __init__(self, name):
        self.name = name

    def send_message(self, message):
        ChatRoom.show_message(self, message)

user1 = User("Alice")
user2 = User("Bob")

user1.send_message("Hi, Bob!") # Output: Alice: Hi, Bob!
user2.send_message("Hello, Alice!") # Output: Bob: Hello, Alice!
```

### Ruby Example

```ruby
class ChatRoom
  def self.show_message(user, message)
    puts "#{user.name}: #{message}"
  end
end

class User
  attr_reader :name

  def initialize(name)
    @name = name
  end

  def send_message(message)
    ChatRoom.show_message(self, message)
  end
end

user1 = User.new("Alice")
user2 = User.new("Bob")

user1.send_message("Hi, Bob!") # Output: Alice: Hi, Bob!
user2.send_message("Hello, Alice!") # Output: Bob: Hello, Alice!
```

### Rust Example

```rust
struct ChatRoom;

impl ChatRoom {
    fn show_message(user: &User, message: &str) {
        println!("{}: {}", user.name, message);
    }
}

struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name }
    }

    fn send_message(&self, message: &str) {
        ChatRoom::show_message(self, message);
    }
}

fn main() {
    let user1 = User::new("Alice");
    let user2 = User::new("Bob");

    user1.send_message("Hi, Bob!"); // Output: Alice: Hi, Bob!
    user2.send_message("Hello, Alice!"); // Output: Bob: Hello, Alice!
}
```
