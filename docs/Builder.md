## Builder Pattern

The builder pattern is a creational design pattern that helps in constructing complex objects step by step. Unlike other creational patterns
that construct objects in a single step, the builder pattern allows for the creation of a complex object through a step-by-step approach, separating
the construction process from its representation. This separation ensures that the same construction process can create different representations of
the object. The builder pattern is particularly useful for creating objects that require numerous parameters or configuration options, ensuring that
the constructed object is always in a valid state.

### Go Example

```go
package main

import "fmt"

type House struct {
    Walls   string
    Roof    string
    Windows string
}

type HouseBuilder struct {
    house House
}

func (b *HouseBuilder) BuildWalls(walls string) *HouseBuilder {
    b.house.Walls = walls
    return b
}

func (b *HouseBuilder) BuildRoof(roof string) *HouseBuilder {
    b.house.Roof = roof
    return b
}

func (b *HouseBuilder) BuildWindows(windows string) *HouseBuilder {
    b.house.Windows = windows
    return b
}

func (b *HouseBuilder) GetHouse() House {
    return b.house
}

// Usage
func main() {
    builder := &HouseBuilder{}
    house := builder.BuildWalls("brick").BuildRoof("tile").BuildWindows("double-glazed").GetHouse()
    fmt.Println(house)
}
```

### Perl Example

```perl
package House;
sub new {
    my $class = shift;
    my $self = {
        walls => undef,
        roof => undef,
        windows => undef,
    };
    bless $self, $class;
    return $self;
}

package HouseBuilder;
sub new {
    my $class = shift;
    my $self = { house => House->new() };
    bless $self, $class;
    return $self;
}

sub build_walls {
    my ($self, $walls) = @_;
    $self->{house}->{walls} = $walls;
    return $self;
}

sub build_roof {
    my ($self, $roof) = @_;
    $self->{house}->{roof} = $roof;
    return $self;
}

sub build_windows {
    my ($self, $windows) = @_;
    $self->{house}->{windows} = $windows;
    return $self;
}

sub get_house {
    my $self = shift;
    return $self->{house};
}

# Usage
my $builder = HouseBuilder->new();
my $house = $builder->build_walls("brick")->build_roof("tile")->build_windows("double-glazed")->get_house();
print "$_ => $house->{$_}\n" for keys %$house;
```

### Python Example

```python
class House:
    def __init__(self):
        self.walls = None
        self.roof = None
        self.windows = None

class HouseBuilder:
    def __init__(self):
        self.house = House()

    def build_walls(self, walls):
        self.house.walls = walls
        return self

    def build_roof(self, roof):
        self.house.roof = roof
        return self

    def build_windows(self, windows):
        self.house.windows = windows
        return self

    def get_house(self):
        return self.house

# Usage
builder = HouseBuilder()
house = builder.build_walls("brick").build_roof("tile").build_windows("double-glazed").get_house()
print(house.__dict__)
```

### Ruby Example

```ruby
class House
  attr_accessor :walls, :roof, :windows

  def initialize
    @walls = nil
    @roof = nil
    @windows = nil
  end
end

class HouseBuilder
  def initialize
    @house = House.new
  end

  def build_walls(walls)
    @house.walls = walls
    self
  end

  def build_roof(roof)
    @house.roof = roof
    self
  end

  def build_windows(windows)
    @house.windows = windows
    self
  end

  def get_house
    @house
  end
end

# Usage
builder = HouseBuilder.new
house = builder.build_walls("brick").build_roof("tile").build_windows("double-glazed").get_house
puts house.inspect
```

### Rust Example

```rust
struct House {
    walls: Option<String>,
    roof: Option<String>,
    windows: Option<String>,
}

struct HouseBuilder {
    house: House,
}

impl HouseBuilder {
    fn new() -> Self {
        HouseBuilder {
            house: House {
                walls: None,
                roof: None,
                windows: None,
            },
        }
    }

    fn build_walls(mut self, walls: &str) -> Self {
        self.house.walls = Some(walls.to_string());
        self
    }

    fn build_roof(mut self, roof: &str) -> Self {
        self.house.roof = Some(roof.to_string());
        self
    }

    fn build_windows(mut self, windows: &str) -> Self {
        self.house.windows = Some(windows.to_string());
        self
    }

    fn get_house(self) -> House {
        self.house
    }
}

// Usage
fn main() {
    let builder = HouseBuilder::new();
    let house = builder
        .build_walls("brick")
        .build_roof("tile")
        .build_windows("double-glazed")
        .get_house();
    println!("{:?}", house);
}
```
