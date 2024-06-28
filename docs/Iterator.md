## Iterator Pattern

The iterator pattern is a behavioural design pattern that provides a way to access the elements of an aggregate object sequentially without
exposing its underlying representation. This pattern encapsulates the iteration logic, allowing for flexible traversal of collections. By
providing a standard interface for traversing different types of collections, the iterator pattern promotes the single responsibility principle,
separating the traversal behaviour from the collection itself. This approach enhances the flexibility and reusability of code, making it easier
to iterate over various data structures without modifying their implementation.


### Go Example

```go
package main

import "fmt"

type MyIterator struct {
    current, end int
}

func NewMyIterator(start, end int) *MyIterator {
    return &MyIterator{current: start, end: end}
}

func (it *MyIterator) Next() (int, bool) {
    if it.current >= it.end {
        return 0, false
    }
    val := it.current
    it.current++
    return val, true
}

func main() {
    iter := NewMyIterator(0, 5)
    for {
        if val, ok = iter.Next(); ok {
            fmt.Println(val)
        } else {
            break
        }
    }
}
```

### Perl Example

```perl
package MyIterator;

sub new {
    my ($class, $start, $end) = @_;
    return bless { current => $start, end => $end }, $class;
}

sub next {
    my $self = shift;
    if ($self->{current} >= $self->{end}) {
        return undef;
    } else {
        return $self->{current}++;
    }
}

# Usage
my $iter = MyIterator->new(0, 5);
while (defined(my $num = $iter->next())) {
    print "$num\n";
}
```

### Python Example

```python
class MyIterator:
    def __init__(self, start, end):
        self.current = start
        self.end = end

    def __iter__(self):
        return self

    def __next__(self):
        if self.current >= self.end:
            raise StopIteration
        else:
            self.current += 1
            return self.current - 1

# Usage
for num in MyIterator(0, 5):
    print(num)
```

### Ruby Example

```ruby
class MyIterator
  include Enumerable

  def initialize(start, stop)
    @start = start
    @stop = stop
  end

  def each
    (@start...@stop).each { |i| yield i }
  end
end

# Usage
MyIterator.new(0, 5).each do |num|
  puts num
end
```

### Rust Example

```rust
struct MyIterator {
    current: usize,
    end: usize,
}

impl MyIterator {
    fn new(start: usize, end: usize) -> MyIterator {
        MyIterator { current: start, end }
    }
}

impl Iterator for MyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            self.current += 1;
            Some(self.current - 1)
        }
    }
}

// Usage
fn main() {
    let iter = MyIterator::new(0, 5);
    for num in iter {
        println!("{}", num);
    }
}
```
