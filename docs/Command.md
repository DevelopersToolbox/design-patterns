## Command Pattern

The command pattern is a behavioural design pattern that turns a request into a stand-alone object containing all the information about the
request. This transformation allows for the parameterization of clients with different requests, queuing or logging requests, and supporting
undoable operations. The command pattern decouples the object that invokes the operation from the one that knows how to perform it. In this
pattern, a command interface declares a method for executing a particular action, while concrete command classes implement this interface,
binding the receiver and the specific actions. This separation allows commands to be composed, stored, and executed dynamically, offering
extensive control over operation flow and facilitating features like undo/redo and transaction management.

### Go Example

```go
package main

import "fmt"

type Command interface {
    Execute()
}

type LightOnCommand struct {
    light Light
}

func (c *LightOnCommand) Execute() {
    c.light.On()
}

type Light struct{}

func (l Light) On() {
    fmt.Println("Light is on")
}

type RemoteControl struct {
    command Command
}

func (r *RemoteControl) SetCommand(command Command) {
    r.command = command
}

func (r *RemoteControl) PressButton() {
    r.command.Execute()
}

func main() {
    light := Light{}
    lightOnCommand := LightOnCommand{light: light}
    remote := RemoteControl{}
    remote.SetCommand(&lightOnCommand)
    remote.PressButton()
}
```

### Perl Example

```perl
package Command;
sub execute { }

package LightOnCommand;
our @ISA = qw(Command);

sub new {
    my ($class, $light) = @_;
    my $self = { light => $light };
    bless $self, $class;
    return $self;
}

sub execute {
    my $self = shift;
    $self->{light}->on();
}

package Light;

sub on {
    print "Light is on\n";
}

package RemoteControl;

sub new {
    my $class = shift;
    my $self = {};
    bless $self, $class;
    return $self;
}

sub set_command {
    my ($self, $command) = @_;
    $self->{command} = $command;
}

sub press_button {
    my $self = shift;
    $self->{command}->execute();
}

# Usage
my $light = Light->new();
my $light_on_command = LightOnCommand->new($light);
my $remote = RemoteControl->new();
$remote->set_command($light_on_command);
$remote->press_button();
```

### Python Example

```python
class Command:
    def execute(self):
        pass

class LightOnCommand(Command):
    def __init__(self, light):
        self.light = light

    def execute(self):
        self.light.on()

class Light:
    def on(self):
        print("Light is on")

class RemoteControl:
    def __init__(self):
        self.command = None

    def set_command(self, command):
        self.command = command

    def press_button(self):
        self.command.execute()

# Usage
light = Light()
light_on_command = LightOnCommand(light)
remote = RemoteControl()
remote.set_command(light_on_command)
remote.press_button()
```

### Ruby Example

```ruby
class Command
  def execute
  end
end

class LightOnCommand < Command
  def initialize(light)
    @light = light
  end

  def execute
    @light.on
  end
end

class Light
  def on
    puts "Light is on"
  end
end

class RemoteControl
  def set_command(command)
    @command = command
  end

  def press_button
    @command.execute
  end
end

# Usage
light = Light.new
light_on_command = LightOnCommand.new(light)
remote = RemoteControl.new
remote.set_command(light_on_command)
remote.press_button
```

### Rust Example

```rust
trait Command {
    fn execute(&self);
}

struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    fn new(light: Light) -> Self {
        LightOnCommand { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.on();
    }
}

struct Light;

impl Light {
    fn on(&self) {
        println!("Light is on");
    }
}

struct RemoteControl {
    command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        RemoteControl { command: None }
    }

    fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    fn press_button(&self) {
        if let Some(ref command) = self.command {
            command.execute();
        }
    }
}

fn main() {
    let light = Light;
    let light_on_command = LightOnCommand::new(light);
    let mut remote = RemoteControl::new();
    remote.set_command(Box::new(light_on_command));
    remote.press_button();
}
```
