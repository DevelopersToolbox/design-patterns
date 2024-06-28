class Singleton
  @instance = nil

  private_class_method :new

  def self.instance
    @instance ||= new
  end
end

# Usage
singleton1 = Singleton.instance
singleton2 = Singleton.instance

puts singleton1.equal?(singleton2)  # Output: true
