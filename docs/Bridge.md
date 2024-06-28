## Bridge Pattern

The bridge pattern is a structural design pattern that decouples an abstraction from its implementation, allowing the two to vary independently.
This pattern involves an abstraction and an implementation class, with a bridge interface connecting them. By separating the abstraction from its
implementation, the bridge pattern promotes flexibility and extensibility, enabling different implementations to be swapped or extended without
affecting the abstraction.

### Go Example

```go
package main

import "fmt"

// Implementor
type Device interface {
	On()
	Off()
}

// Concrete Implementor 1
type TV struct{}

func (t *TV) On() {
	fmt.Println("TV is On")
}

func (t *TV) Off() {
	fmt.Println("TV is Off")
}

// Concrete Implementor 2
type Radio struct{}

func (r *Radio) On() {
	fmt.Println("Radio is On")
}

func (r *Radio) Off() {
	fmt.Println("Radio is Off")
}

// Abstraction
type RemoteControl struct {
	device Device
}

func (r *RemoteControl) TurnOn() {
	r.device.On()
}

func (r *RemoteControl) TurnOff() {
	r.device.Off()
}

func main() {
	tv := &TV{}
	radio := &Radio{}

	tvRemote := &RemoteControl{device: tv}
	radioRemote := &RemoteControl{device: radio}

	tvRemote.TurnOn()    // Output: TV is On
	tvRemote.TurnOff()   // Output: TV is Off
	radioRemote.TurnOn() // Output: Radio is On
	radioRemote.TurnOff() // Output: Radio is Off
}
```

### Perl Example

```perl
package Device;

use strict;
use warnings;

sub on { die "Abstract method" }
sub off { die "Abstract method" }

package TV;
use parent 'Device';

sub on {
    print "TV is On\n";
}

sub off {
    print "TV is Off\n";
}

package Radio;
use parent 'Device';

sub on {
    print "Radio is On\n";
}

sub off {
    print "Radio is Off\n";
}

package RemoteControl;

sub new {
    my ($class, $device) = @_;
    return bless { device => $device }, $class;
}

sub turn_on {
    my $self = shift;
    $self->{device}->on();
}

sub turn_off {
    my $self = shift;
    $self->{device}->off();
}

package main;

my $tv = TV->new();
my $radio = Radio->new();

my $tv_remote = RemoteControl->new($tv);
my $radio_remote = RemoteControl->new($radio);

$tv_remote->turn_on();    # Output: TV is On
$tv_remote->turn_off();   # Output: TV is Off
$radio_remote->turn_on(); # Output: Radio is On
$radio_remote->turn_off(); # Output: Radio is Off
```

### Python Example

```python
class Device:
    def on(self):
        raise NotImplementedError

    def off(self):
        raise NotImplementedError

class TV(Device):
    def on(self):
        print("TV is On")

    def off(self):
        print("TV is Off")

class Radio(Device):
    def on(self):
        print("Radio is On")

    def off(self):
        print("Radio is Off")

class RemoteControl:
    def __init__(self, device):
        self.device = device

    def turn_on(self):
        self.device.on()

    def turn_off(self):
        self.device.off()

tv = TV()
radio = Radio()

tv_remote = RemoteControl(tv)
radio_remote = RemoteControl(radio)

tv_remote.turn_on()    # Output: TV is On
tv_remote.turn_off()   # Output: TV is Off
radio_remote.turn_on() # Output: Radio is On
radio_remote.turn_off() # Output: Radio is Off
```

### Ruby Example

```ruby
class Device
  def on
    raise 'Abstract method'
  end

  def off
    raise 'Abstract method'
  end
end

class TV < Device
  def on
    puts 'TV is On'
  end

  def off
    puts 'TV is Off'
  end
end

class Radio < Device
  def on
    puts 'Radio is On'
  end

  def off
    puts 'Radio is Off'
  end
end

class RemoteControl
  def initialize(device)
    @device = device
  end

  def turn_on
    @device.on
  end

  def turn_off
    @device.off
  end
end

tv = TV.new
radio = Radio.new

tv_remote = RemoteControl.new(tv)
radio_remote = RemoteControl.new(radio)

tv_remote.turn_on    # Output: TV is On
tv_remote.turn_off   # Output: TV is Off
radio_remote.turn_on # Output: Radio is On
radio_remote.turn_off # Output: Radio is Off
```

### Rust Example

```rust
trait Device {
    fn on(&self);
    fn off(&self);
}

struct TV;

impl Device for TV {
    fn on(&self) {
        println!("TV is On");
    }

    fn off(&self) {
        println!("TV is Off");
    }
}

struct Radio;

impl Device for Radio {
    fn on(&self) {
        println!("Radio is On");
    }

    fn off(&self) {
        println!("Radio is Off");
    }
}

struct RemoteControl {
    device: Box<dyn Device>,
}

impl RemoteControl {
    fn new(device: Box<dyn Device>) -> Self {
        RemoteControl { device }
    }

    fn turn_on(&self) {
        self.device.on();
    }

    fn turn_off(&self) {
        self.device.off();
    }
}

fn main() {
    let tv = Box::new(TV);
    let radio = Box::new(Radio);

    let tv_remote = RemoteControl::new(tv);
    let radio_remote = RemoteControl::new(radio);

    tv_remote.turn_on();    // Output: TV is On
    tv_remote.turn_off();   // Output: TV is Off
    radio_remote.turn_on(); // Output: Radio is On
    radio_remote.turn_off(); // Output: Radio is Off
}
```
