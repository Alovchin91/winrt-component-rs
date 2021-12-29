var sample = new RustComponent.Sample();
sample.Greeting = "Hello from Rust!";
Console.WriteLine(sample.Greeting);

var sample2 = new RustComponent.Sample("Made by factory");
sample2.PrintGreeting();
