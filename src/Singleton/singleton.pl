package Singleton;

my $instance;

sub new {
    my $class = shift;
    unless (defined $instance) {
        $instance = bless {}, $class;
    }
    return $instance;
}

# Usage
my $singleton1 = Singleton->new();
my $singleton2 = Singleton->new();

print $singleton1 == $singleton2 ? "true\n" : "false\n";  # Output: true
