## Factory Pattern

The factory pattern is a creational design pattern that provides an interface for creating objects while allowing subclasses to decide which
class to instantiate. This pattern promotes loose coupling by eliminating the need to bind application-specific classes into the code. The
factory pattern encapsulates the object creation process, enabling flexibility and reusability. It is particularly useful when the exact type
of the object cannot be determined until runtime, as it provides a way to defer instantiation to subclasses, ensuring that the appropriate
object is created for a given situation.

### Go Example

```go
package main

import "fmt"

type Animal interface {
    Speak() string
}

type Dog struct{}
func (d Dog) Speak() string {
    return "Woof!"
}

type Cat struct{}
func (c Cat) Speak() string {
    return "Meow!"
}

type AnimalFactory struct{}
func (af AnimalFactory) GetAnimal(animalType string) Animal {
    switch animalType {
    case "dog":
        return Dog{}
    case "cat":
        return Cat{}
    default:
        return nil
    }
}

func main() {
    factory := AnimalFactory{}
    animal := factory.GetAnimal("dog")
    if animal != nil {
        fmt.Println(animal.Speak())
    } else {
        fmt.Println("Unknown animal type")
    }
}
```

### Perl Example

```perl
package Animal;
sub speak {
    die "You must implement the speak method";
}

package Dog;
use parent 'Animal';
sub speak {
    return "Woof!";
}

package Cat;
use parent 'Animal';
sub speak {
    return "Meow!";
}

package AnimalFactory;
sub get_animal {
    my ($class, $animal_type) = @_;
    if ($animal_type eq 'dog') {
        return Dog->new();
    } elsif ($animal_type eq 'cat') {
        return Cat->new();
    } else {
        return undef;
    }
}

# Usage
my $factory = 'AnimalFactory';
my $animal = $factory->get_animal('dog');
print $animal->speak(), "\n";
```

### Python Example

```python
from abc import ABC, abstractmethod

class Animal(ABC):
    @abstractmethod
    def speak(self):
        pass

class Dog(Animal):
    def speak(self):
        return "Woof!"

class Cat(Animal):
    def speak(self):
        return "Meow!"

class AnimalFactory:
    @staticmethod
    def get_animal(animal_type):
        if animal_type == "dog":
            return Dog()
        elif animal_type == "cat":
            return Cat()
        else:
            return None

# Usage
factory = AnimalFactory()
animal = factory.get_animal("dog")
print(animal.speak())
```

### Ruby Example

```ruby
class Animal
  def speak
    raise NotImplementedError, 'You must implement the speak method'
  end
end

class Dog < Animal
  def speak
    'Woof!'
  end
end

class Cat < Animal
  def speak
    'Meow!'
  end
end

class AnimalFactory
  def self.get_animal(animal_type)
    case animal_type
    when 'dog'
      Dog.new
    when 'cat'
      Cat.new
    else
      nil
    end
  end
end

# Usage
factory = AnimalFactory
animal = factory.get_animal('dog')
puts animal.speak
```

### Rust Example

```rust
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

enum AnimalType {
    Dog,
    Cat,
}

struct AnimalFactory;

impl AnimalFactory {
    fn get_animal(animal_type: AnimalType) -> Box<dyn Animal> {
        match animal_type {
            AnimalType::Dog => Box::new(Dog),
            AnimalType::Cat => Box::new(Cat),
        }
    }
}

fn main() {
    let factory = AnimalFactory;
    let animal = factory.get_animal(AnimalType::Dog);
    println!("{}", animal.speak());
}
```
